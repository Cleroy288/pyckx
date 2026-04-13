//! Constants module
//!
//! Centralized constants for the application.
//!
//! # Structure
//! - `errors/` - Global error aggregator (AppError, ErrorResponse)
//! - `urls/` - External service URL constants

pub mod errors;
pub mod urls;

// Re-export commonly used types
pub use errors::{AppError, AppResult};
