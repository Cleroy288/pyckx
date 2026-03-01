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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stores_message() {
        // arrange / act
        let err = InternalError::new("boom");

        // assert
        assert_eq!(err.0, "boom");
    }

    #[test]
    fn test_code_always_internal_error() {
        // arrange
        let err = InternalError::new("x");

        // act / assert
        assert_eq!(err.code(), "INTERNAL_ERROR");
    }

    #[test]
    fn test_message_always_internal_server_error() {
        // arrange
        let err = InternalError::new("x");

        // act / assert
        assert_eq!(err.message(), "Internal server error");
    }

    #[test]
    fn test_status_always_500() {
        // arrange
        let err = InternalError::new("x");

        // act / assert
        assert_eq!(
            err.status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_display_includes_message() {
        // arrange
        let err = InternalError::new("db crash");

        // act / assert
        assert_eq!(
            err.to_string(),
            "Internal error: db crash"
        );
    }
}
