//! HTTP API layer - Web interface for the application.
//!
//! This layer handles all HTTP concerns: routing, request parsing,
//! response formatting, and middleware.
//!
//! # Structure
//! - `handlers/` - Route handlers organized by feature
//! - `data_transfer_object/` - Request/Response data transfer objects
//! - `middlewares/` - Custom middleware (rate limiting, etc.)
//! - `utils/` - Validation and other utilities

pub mod data_transfer_object;
pub mod handlers;
pub mod middlewares;
pub mod utils;

pub use handlers::init;
pub use middlewares::{RateLimitConfig, RateLimitMiddleware, RateLimiter};
