//! Rate Limiting Middleware
//!
//! Provides configurable rate limiting per route with multiple time windows.
//!
//! # Example
//! ```ignore
//! use crate::http_api::middlewares::rate_limit::{RateLimitConfig, RateLimiter};
//!
//! let limiter = RateLimiter::new();
//!
//! // Configure: 10 requests/second, 100/minute, 1000/hour
//! limiter.configure("/api/login", RateLimitConfig::new()
//!     .per_second(10)
//!     .per_minute(100)
//!     .per_hour(1000));
//! ```

mod config;
mod error;
mod middleware;
mod store;

pub use config::RateLimitConfig;
pub use middleware::RateLimitMiddleware;
pub use store::RateLimiter;
