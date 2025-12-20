//! User handlers module
//!
//! User-related endpoints (current user info).

mod helpers;
mod me;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/user").service(me::me_handler));
}
