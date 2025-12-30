//! Apps handlers module
//!
//! App and user app management endpoints.

mod get_apps;
mod manage_apps;
mod user_apps;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/apps")
            .service(get_apps::get_all_apps_handler)
            .service(get_apps::get_app_handler)
            .service(manage_apps::create_app_handler)
            .service(manage_apps::update_app_handler)
            .service(manage_apps::delete_app_handler),
    )
    .service(
        web::scope("/api/user/apps")
            .service(user_apps::get_user_apps_handler)
            .service(user_apps::add_user_app_handler)
            .service(user_apps::remove_user_app_handler),
    );
}
