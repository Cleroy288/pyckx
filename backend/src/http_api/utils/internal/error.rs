//! Internal error
//!
//! Self-contained error for internal server errors.

use actix_web::http::StatusCode;
use std::fmt;

/// Internal server error
#[derive(Debug, Clone)]
pub struct InternalError(pub String);

impl InternalError {
    /// Create a new internal error
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        "INTERNAL_ERROR"
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        "Internal server error"
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Internal error: {}", self.0)
    }
}

impl std::error::Error for InternalError {}
