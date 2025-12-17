//! App error module
//!
//! Top-level application error with ResponseError implementation.

mod conversions;
mod error;

pub use error::{AppError, AppResult};
