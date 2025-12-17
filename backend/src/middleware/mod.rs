//! Middleware module - Cross-cutting concerns
//!
//! Contains middleware for:
//! - Rate limiting (`rate_limit/`)

pub mod rate_limit;

// == Rate Limiting Exports // ==
pub use rate_limit::{RateLimitConfig, RateLimitMiddleware, RateLimiter};
