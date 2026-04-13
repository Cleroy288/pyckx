//! Auth handlers module
//!
//! Routes are proxied to the auth microservice (Hono/Bun).

mod helpers;
mod login;
mod logout;
mod register;

use super::auth_proxy;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/login", web::post().to(auth_proxy::proxy))
            .route("/logout", web::post().to(auth_proxy::proxy))
            .route("/register", web::post().to(auth_proxy::proxy)),
    );
}
