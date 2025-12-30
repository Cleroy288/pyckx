//! Collection handlers module
//!
//! Collection and DVD management endpoints.

mod collection_handlers;
mod dvd_handlers;
mod helpers;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/collection")
            // DVD operations (specific routes BEFORE wildcards)
            .service(dvd_handlers::add_dvd_handler)
            .service(dvd_handlers::get_user_dvds_handler)
            .service(dvd_handlers::get_dvd_handler)
            .service(dvd_handlers::update_dvd_handler)
            .service(dvd_handlers::delete_dvd_handler)
            // Collection management (wildcard routes LAST)
            .service(collection_handlers::create_collection_handler)
            .service(collection_handlers::get_collections_handler)
            .service(collection_handlers::get_collection_items_handler)
            .service(collection_handlers::delete_collection_handler),
    );
}
