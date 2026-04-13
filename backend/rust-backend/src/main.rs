// ============================================================================
// LAPP - Login Application
// ============================================================================

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_multipart::form::MultipartFormConfig;
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    rt::signal,
    web, App as ActixApp, HttpRequest, HttpServer,
};
use LAPP::app::App;
use LAPP::http_api::{RateLimitConfig, RateLimitMiddleware, RateLimiter};
use tracing::{error, info, warn};
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

// ============================================================================
// MAIN
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    // Initialize application
    let app = match App::new().await {
        Ok(app) => app,
        Err(err) => {
            error!("Failed to load configuration: {}", err);
            eprintln!("\n❌ Configuration Error: {}", err);
            eprintln!("\nRequired env vars: IP, PORT, SP_ID, SP_URL, SP_ANON, SP_SERVICE_ROLE, SECURE_HTTP");
            std::process::exit(1);
        }
    };

    info!(
        name = %app.name, version = %app.version,
        ip = %app.config.ip, port = %app.config.port,
        serve_frontend = %app.config.serve_frontend,
        "Starting server"
    );

    let port: u16 = app.config.port.parse().unwrap_or_else(|_| {
        error!("Invalid PORT: {}", app.config.port);
        std::process::exit(1);
    });

    let app_data = web::Data::new(app.clone());
    let rate_limiter = configure_rate_limiter();
    let serve_frontend = app.config.serve_frontend;
    let static_dir = app.config.static_dir.clone();

    if serve_frontend {
        info!(static_dir = %static_dir, "Frontend serving enabled");
    }

    // Build server
    let server = HttpServer::new(move || {
        let multipart_config = MultipartFormConfig::default()
            .total_limit(100 * 1024 * 1024)
            .memory_limit(50 * 1024 * 1024);

        let json_config = web::JsonConfig::default().limit(10 * 1024 * 1024);
        let payload_config =
            web::PayloadConfig::default().limit(100 * 1024 * 1024);

        let cors = if serve_frontend {
            Cors::default()
        } else {
            Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec![
                    "GET", "POST", "PUT", "DELETE", "OPTIONS",
                ])
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                ])
                .supports_credentials()
                .max_age(3600)
        };

        let mut actix_app = ActixApp::new()
            .app_data(app_data.clone())
            .app_data(multipart_config)
            .app_data(json_config)
            .app_data(payload_config)
            .wrap(cors)
            .wrap(Compress::default())
            .wrap(RateLimitMiddleware::new(rate_limiter.clone()))
            .wrap(Logger::new("%a \"%r\" %s %b %Dms"))
            .configure(LAPP::http_api::init);

        // Static file serving (Leptos WASM app)
        if serve_frontend {
            let static_dir_clone = static_dir.clone();
            actix_app = actix_app
                // Serve all static files with SPA fallback
                .service(
                    Files::new("/", static_dir.clone())
                        .index_file("index.html")
                        .prefer_utf8(true)
                        .default_handler(web::to(move |req: HttpRequest| {
                            let static_dir = static_dir_clone.clone();
                            async move { spa_fallback(req, static_dir).await }
                        })),
                );
        }

        actix_app
    })
    .bind((app.config.ip.clone(), port))?
    .run();

    let scheme = if app.config.secure_http == "true" {
        "https"
    } else {
        "http"
    };
    println!(
        "\n🌐 {} running at {}://{}:{}\n",
        app.name, scheme, app.config.ip, port
    );

    // Graceful shutdown
    let srv_handle = server.handle();
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        info!("Shutting down...");
        srv_handle.stop(true).await;
    });

    server.await
}

// ============================================================================
// HELPERS
// ============================================================================

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new(
                "info,actix_web=info,actix_server=info",
            )
        });

    // File appender: logs/pyckx.log (daily rotation)
    let file_appender = tracing_appender::rolling::daily(
        "logs", "pyckx.log",
    );
    let (writer, _guard) = tracing_appender::non_blocking(
        file_appender,
    );
    // Leak the guard so it lives for the process lifetime
    std::mem::forget(_guard);

    tracing_subscriber::registry()
        .with(filter)
        // Stdout: compact for dev readability
        .with(fmt::layer().with_target(true).compact())
        // File: JSON for machine parsing
        .with(
            fmt::layer()
                .json()
                .with_writer(writer)
                .with_target(true),
        )
        .init();
}

fn configure_rate_limiter() -> RateLimiter {
    let limiter = RateLimiter::with_default(RateLimitConfig::standard());

    // Auth - strict limits
    limiter.configure("/api/auth/login", RateLimitConfig::strict());
    limiter.configure("/api/auth/register", RateLimitConfig::strict());

    // Collection - relaxed limits
    limiter.configure("/api/collection/dvds", RateLimitConfig::relaxed());

    // AI Generation - 1 per minute to prevent abuse
    let ai_config = RateLimitConfig::ai_generation();
    limiter.configure("/api/study/qcm/generate", ai_config.clone());
    limiter.configure("/api/study/open-questions", ai_config.clone());
    limiter.configure("/api/study/flashcards", ai_config.clone());
    limiter.configure("/api/study/true-false", ai_config.clone());
    limiter.configure("/api/study/keywords", ai_config.clone());
    limiter.configure("/api/study/order-phrases", ai_config.clone());
    limiter.configure("/api/study/fill-blanks", ai_config.clone());
    limiter.configure("/api/study/generate-course", ai_config);

    limiter
}

/// SPA fallback - serves index.html for unmatched routes (client-side routing)
async fn spa_fallback(
    req: HttpRequest,
    static_dir: String,
) -> actix_web::Result<NamedFile> {
    let path = req.path();

    // Reject API routes that fell through
    if path.starts_with("/api/") {
        warn!(path = %path, "API route not found");
        return Err(actix_web::error::ErrorNotFound("API endpoint not found"));
    }

    // Try route-specific index.html (e.g., /study -> /study/index.html)
    let route_index =
        format!("{}{}/index.html", static_dir, path.trim_end_matches('/'));
    if let Ok(file) = NamedFile::open(&route_index) {
        return Ok(file);
    }

    // Fallback to root index.html for SPA client-side routing
    NamedFile::open(format!("{}/index.html", static_dir))
        .map_err(|_| actix_web::error::ErrorNotFound("Frontend not found"))
}
