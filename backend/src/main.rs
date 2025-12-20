// ============================================================================
// LAPP - Login Application
// ============================================================================

// Modules
mod api;
mod app;
mod apps;
mod config;
mod domain;
mod error;
mod infrastructure;
mod middleware;
mod services;
mod shared;
mod use_cases;

#[cfg(test)]
mod tests;

// Imports
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_multipart::form::MultipartFormConfig;
use actix_web::{http::header, middleware::Logger, rt::signal, web, App as ActixApp, HttpRequest, HttpServer};
use app::App;
use middleware::{RateLimitConfig, RateLimitMiddleware, RateLimiter};
use tracing::{error, info, warn};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// ============================================================================
// MAIN
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    // Initialize application
    let app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            eprintln!("\n❌ Configuration Error: {}", e);
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
        let payload_config = web::PayloadConfig::default().limit(100 * 1024 * 1024);

        let cors = if serve_frontend {
            Cors::default()
        } else {
            Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
                .supports_credentials()
                .max_age(3600)
        };

        let mut actix_app = ActixApp::new()
            .app_data(app_data.clone())
            .app_data(multipart_config)
            .app_data(json_config)
            .app_data(payload_config)
            .wrap(cors)
            .wrap(RateLimitMiddleware::new(rate_limiter.clone()))
            .wrap(Logger::new("%a \"%r\" %s %b %Dms"))
            .configure(api::init);

        // Static file serving (production only)
        if serve_frontend {
            let static_dir_clone = static_dir.clone();
            actix_app = actix_app
                // Serve /_next/* for JS/CSS bundles
                .service(Files::new("/_next", format!("{}/_next", static_dir)).prefer_utf8(true))
                // Serve all static files with automatic index.html per directory
                .service(
                    Files::new("/", static_dir.clone())
                        .index_file("index.html")
                        .prefer_utf8(true)
                        .default_handler(web::to(move |req: HttpRequest| {
                            let static_dir = static_dir_clone.clone();
                            async move { spa_fallback(req, static_dir).await }
                        }))
                );
        }

        actix_app
    })
    .bind((app.config.ip.clone(), port))?
    .run();

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
        .unwrap_or_else(|_| EnvFilter::new("info,actix_web=info,actix_server=info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).compact())
        .init();
}

fn configure_rate_limiter() -> RateLimiter {
    let limiter = RateLimiter::with_default(RateLimitConfig::standard());
    limiter.configure("/api/auth/login", RateLimitConfig::strict());
    limiter.configure("/api/auth/register", RateLimitConfig::strict());
    limiter.configure("/api/collection/dvds", RateLimitConfig::relaxed());
    limiter
}

/// SPA fallback - serves index.html for unmatched routes (client-side routing)
async fn spa_fallback(req: HttpRequest, static_dir: String) -> actix_web::Result<NamedFile> {
    let path = req.path();

    // Reject API routes that fell through
    if path.starts_with("/api/") {
        warn!(path = %path, "API route not found");
        return Err(actix_web::error::ErrorNotFound("API endpoint not found"));
    }

    // Try route-specific index.html (e.g., /intello -> /intello/index.html)
    let route_index = format!("{}{}/index.html", static_dir, path.trim_end_matches('/'));
    if let Ok(file) = NamedFile::open(&route_index) {
        return Ok(file);
    }

    // Fallback to root index.html for SPA client-side routing
    NamedFile::open(format!("{}/index.html", static_dir))
        .map_err(|_| actix_web::error::ErrorNotFound("Frontend not found"))
}

