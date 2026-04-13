//! Validation error
//!
//! Self-contained error for input validation failures.

use actix_web::http::StatusCode;

/// Validation error for input data
#[derive(Debug, Clone, thiserror::Error)]
#[error("Validation error on '{field}': {message}")]
pub struct ValidationError {
    /// Field that failed validation
    pub field: &'static str,
    /// Detail about what failed
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(
        field: &'static str,
        message: impl Into<String>,
    ) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stores_field_and_message() {
        let err = ValidationError::new("email", "invalid");
        assert_eq!(err.field, "email");
        assert_eq!(err.message, "invalid");
    }

    #[test]
    fn test_code_always_validation_failed() {
        let err = ValidationError::new("name", "x");
        assert_eq!(err.code(), "VALIDATION_FAILED");
    }

    #[test]
    fn test_message_always_invalid_input() {
        let err = ValidationError::new("name", "x");
        assert_eq!(err.message(), "Invalid input data");
    }

    #[test]
    fn test_status_always_bad_request() {
        let err = ValidationError::new("name", "x");
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_display_includes_field_and_detail() {
        let err =
            ValidationError::new("password", "too short");
        assert_eq!(
            err.to_string(),
            "Validation error on 'password': too short"
        );
    }
}
