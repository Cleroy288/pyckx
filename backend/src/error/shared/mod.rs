//! Shared error types
//!
//! Common types used across all error modules.

mod code;
mod response;

pub use code::ErrorCode;
pub use response::ErrorResponse;
