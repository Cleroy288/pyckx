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
    // == INITIALIZE TRACING // ==
    init_tracing();

    // == INITIALIZE APPLICATION // ==
    let app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            eprintln!("\n❌ Configuration Error: {}", e);
            eprintln!("\nPlease ensure all required environment variables are set in .env:");
            eprintln!("  - IP");
            eprintln!("  - PORT");
            eprintln!("  - SP_ID");
            eprintln!("  - SP_URL");
            eprintln!("  - SP_ANON");
            eprintln!("  - SP_SERVICE_ROLE");
            eprintln!("  - SECURE_HTTP");
            eprintln!("\nSee .env.example for reference.\n");
            std::process::exit(1);
        }
    };

    info!(
        name = %app.name,
        version = %app.version,
        ip = %app.config.ip,
        port = %app.config.port,
        serve_frontend = %app.config.serve_frontend,
        "Starting server"
    );

    // == PARSE PORT // ==
    let port: u16 = app.config.port.parse().unwrap_or_else(|_| {
        error!("Invalid PORT value: {}", app.config.port);
        eprintln!("❌ Invalid PORT value: '{}'. Must be a number between 1-65535.", app.config.port);
        std::process::exit(1);
    });

    // Share app state across handlers
    let app_data = web::Data::new(app.clone());

    // == CONFIGURE RATE LIMITER // ==
    let rate_limiter = configure_rate_limiter();
    info!("Rate limiter configured");

    // == FRONTEND SERVING CONFIGURATION // ==
    let serve_frontend = app.config.serve_frontend;
    let static_dir = app.config.static_dir.clone();

    if serve_frontend {
        info!(static_dir = %static_dir, "Frontend serving enabled");
    } else {
        info!("Frontend serving disabled (development mode)");
    }

    // == BUILD AND RUN SERVER // ==
    let server = HttpServer::new(move || {
        // Configure multipart payload limits (100MB max for document uploads)
        let multipart_config = MultipartFormConfig::default()
            .total_limit(100 * 1024 * 1024)  // 100MB total
            .memory_limit(50 * 1024 * 1024); // 50MB in memory

        // Configure JSON payload limit
        let json_config = web::JsonConfig::default()
            .limit(10 * 1024 * 1024); // 10MB for JSON

        // Configure general payload limit
        let payload_config = web::PayloadConfig::default()
            .limit(100 * 1024 * 1024); // 100MB

        // Configure CORS for development (when frontend runs on different port)
        // In production, frontend is served by backend so CORS is not needed
        let cors = if serve_frontend {
            // Production: restrictive CORS (same-origin)
            Cors::default()
        } else {
            // Development: allow frontend dev server
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
            // CORS middleware (must be before other middleware)
            .wrap(cors)
            // Rate limiting middleware (before Logger to block early)
            .wrap(RateLimitMiddleware::new(rate_limiter.clone()))
            // Request logging middleware
            .wrap(Logger::new("%a \"%r\" %s %b %Dms"))
            // Configure API routes (highest priority)
            .configure(api::init);

        // == STATIC FILE SERVING (production only) // ==
        if serve_frontend {
            let static_dir_clone = static_dir.clone();
            actix_app = actix_app
                // Serve Next.js static assets (JS, CSS, etc.)
                .service(Files::new("/_next", format!("{}/_next", static_dir)).prefer_utf8(true))
                // SPA fallback: serve index.html for all unmatched routes
                .default_service(web::route().to(move |req: HttpRequest| {
                    let static_dir = static_dir_clone.clone();
                    async move { spa_fallback(req, static_dir).await }
                }));
        }

        actix_app
    })
    .bind((app.config.ip.clone(), port))?
    .run();

    // == GRACEFUL SHUTDOWN // ==
    let srv_handle = server.handle();
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        info!("Received CTRL+C, shutting down gracefully...");
        srv_handle.stop(true).await;
    });

    server.await
}

// ============================================================================
// TRACING SETUP
// ============================================================================

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,actix_web=info,actix_server=info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .compact(),
        )
        .init();
}

// ============================================================================
// RATE LIMITER CONFIGURATION
// ============================================================================

/// Configure rate limits for all routes
fn configure_rate_limiter() -> RateLimiter {
    // Default: standard limits for all routes
    let limiter = RateLimiter::with_default(RateLimitConfig::standard());

    // == AUTH ENDPOINTS (strict) // ==
    limiter.configure("/api/auth/login", RateLimitConfig::strict());
    limiter.configure("/api/auth/register", RateLimitConfig::strict());
    limiter.configure("/api/auth/logout", RateLimitConfig::standard());

    // == COLLECTION READ ENDPOINTS (relaxed) // ==
    limiter.configure_many(
        &[
            "/app/collection/get-user-dvd",
            "/app/collection/dvd",
        ],
        RateLimitConfig::relaxed(),
    );

    // == COLLECTION WRITE ENDPOINTS (standard) // ==
    limiter.configure_many(
        &[
            "/app/collection/add-dvd",
            "/app/collection/mod-dvd",
            "/app/collection/del",
        ],
        RateLimitConfig::standard(),
    );

    limiter
}

// ============================================================================
// SPA FALLBACK HANDLER
// ============================================================================

/// Serve index.html for SPA client-side routing
/// This handles all routes not matched by API handlers or static files
async fn spa_fallback(req: HttpRequest, static_dir: String) -> actix_web::Result<NamedFile> {
    let path = req.path();
    
    // Handle root path - serve index.html directly
    if path == "/" {
        let index_path = format!("{}/index.html", static_dir);
        return match NamedFile::open(&index_path) {
            Ok(file) => Ok(file),
            Err(e) => {
                warn!(path = %path, error = %e, "Failed to serve index.html");
                Err(actix_web::error::ErrorNotFound("Frontend not found"))
            }
        };
    }
    
    // Try to serve the exact file first (for direct file requests like /icon.svg)
    let file_path = format!("{}{}", static_dir, path);
    if let Ok(file) = NamedFile::open(&file_path) {
        return Ok(file);
    }
    
    // Try with .html extension (Next.js static export with trailingSlash)
    // e.g., /login/ -> /login.html or /login/index.html
    let path_trimmed = path.trim_end_matches('/');
    
    // Try path.html (e.g., /login -> /login.html)
    let html_path = format!("{}{}.html", static_dir, path_trimmed);
    if let Ok(file) = NamedFile::open(&html_path) {
        return Ok(file);
    }
    
    // Try path/index.html (e.g., /login/ -> /login/index.html)
    let index_in_dir = format!("{}{}/index.html", static_dir, path_trimmed);
    if let Ok(file) = NamedFile::open(&index_in_dir) {
        return Ok(file);
    }
    
    // Fallback to root index.html for SPA client-side routing
    let index_path = format!("{}/index.html", static_dir);
    match NamedFile::open(&index_path) {
        Ok(file) => Ok(file),
        Err(e) => {
            warn!(path = %path, error = %e, "Failed to serve index.html");
            Err(actix_web::error::ErrorNotFound("Frontend not found"))
        }
    }
}
