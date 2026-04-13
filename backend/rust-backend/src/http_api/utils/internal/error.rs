//! Internal error
//!
//! Self-contained error for internal server errors.

use actix_web::http::StatusCode;

/// Internal server error
#[derive(Debug, Clone, thiserror::Error)]
#[error("Internal error: {0}")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stores_message() {
        let err = InternalError::new("boom");
        assert_eq!(err.0, "boom");
    }

    #[test]
    fn test_code_always_internal_error() {
        let err = InternalError::new("x");
        assert_eq!(err.code(), "INTERNAL_ERROR");
    }

    #[test]
    fn test_message_always_internal_server_error() {
        let err = InternalError::new("x");
        assert_eq!(err.message(), "Internal server error");
    }

    #[test]
    fn test_status_always_500() {
        let err = InternalError::new("x");
        assert_eq!(
            err.status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_display_includes_message() {
        let err = InternalError::new("db crash");
        assert_eq!(
            err.to_string(),
            "Internal error: db crash"
        );
    }
}
