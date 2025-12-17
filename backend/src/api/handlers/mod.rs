//! HTTP handlers - Route handlers organized by feature
//!
//! # Structure
//! - `auth/` - Authentication endpoints (/auth/*)
//! - `user/` - User endpoints (/user/*)
//! - `collection/` - Collection endpoints (/app/collection/*)
//! - `apps/` - Apps endpoints (/api/apps/*, /api/user/apps/*)
//! - `intello/` - Intello endpoints (/app/intello/*)

pub mod apps;
pub mod auth;
pub mod collection;
pub mod intello;
pub mod user;

use actix_web::web;

/// Initialize all API routes
pub fn init(cfg: &mut web::ServiceConfig) {
    auth::init(cfg);
    user::init(cfg);
    collection::init(cfg);
    apps::init(cfg);
    intello::init(cfg);
}
