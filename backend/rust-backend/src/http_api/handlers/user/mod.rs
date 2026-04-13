//! User handlers module
//!
//! /api/user/me is proxied to the auth microservice.

mod helpers;
mod me;

use super::auth_proxy;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/user")
            .route("/me", web::get().to(auth_proxy::proxy)),
    );
}
