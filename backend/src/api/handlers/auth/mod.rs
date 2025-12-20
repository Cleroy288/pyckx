//! Auth handlers module
//!
//! Authentication endpoints: login, register, logout.

mod helpers;
mod login;
mod logout;
mod register;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .service(login::login_handler)
            .service(register::register_handler)
            .service(logout::logout_handler),
    );
}
