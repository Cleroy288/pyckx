//! Validation error
//!
//! Self-contained error for input validation failures.

use actix_web::http::StatusCode;
use std::fmt;

/// Validation error for input data
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: &'static str,
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(field: &'static str, message: impl Into<String>) -> Self {
        Self {
            field,
            message: message.into(),
        }
    }

    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        "VALIDATION_FAILED"
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        "Invalid input data"
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation error on '{}': {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}
