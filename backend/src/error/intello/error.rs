//! Intello error - Service layer errors for Intello app operations
//!
//! These errors are thrown by the IntelloService and converted
//! to AppError for HTTP responses.

use crate::error::shared::ErrorCode;
use std::fmt;

/// Errors that can occur during Intello operations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum IntelloError {
    /// Game set not found (generic, works for any game type)
    GameSetNotFound { game_type: String, set_id: String },
    /// Validation failed
    ValidationFailed { field: String, message: String },
    /// Storage operation failed
    StorageError { message: String },
    /// External service error (AI, API calls)
    ExternalServiceError { service: String, message: String },
    /// Resource not found
    NotFound,
    /// Access forbidden (ownership violation)
    Forbidden,
    /// Resource conflict (duplicate)
    Conflict(String),
}

#[allow(dead_code)]
impl IntelloError {
    /// Get the error code for this error
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::GameSetNotFound { .. } => ErrorCode::IntelloGameSetNotFound,
            Self::ValidationFailed { .. } => ErrorCode::IntelloValidationFailed,
            Self::StorageError { .. } => ErrorCode::IntelloStorageError,
            Self::ExternalServiceError { .. } => ErrorCode::IntelloStorageError,
            Self::NotFound => ErrorCode::IntelloGameSetNotFound,
            Self::Forbidden => ErrorCode::IntelloValidationFailed,
            Self::Conflict(_) => ErrorCode::IntelloValidationFailed,
        }
    }

    /// Convert to HTTP status code
    pub fn status_code(&self) -> u16 {
        match self {
            Self::GameSetNotFound { .. } => 404,
            Self::ValidationFailed { .. } => 400,
            Self::StorageError { .. } => 500,
            Self::ExternalServiceError { .. } => 502,
            Self::NotFound => 404,
            Self::Forbidden => 403,
            Self::Conflict(_) => 409,
        }
    }

    /// Create a GameSetNotFound error (generic, works for any game type)
    pub fn game_not_found(game_type: impl Into<String>, set_id: impl Into<String>) -> Self {
        Self::GameSetNotFound { 
            game_type: game_type.into(), 
            set_id: set_id.into() 
        }
    }

    /// Create a ValidationFailed error
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationFailed { field: field.into(), message: message.into() }
    }

    /// Create a StorageError
    pub fn storage(message: impl Into<String>) -> Self {
        Self::StorageError { message: message.into() }
    }

    /// Create an ExternalServiceError
    pub fn external(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalServiceError { service: service.into(), message: message.into() }
    }
}

impl fmt::Display for IntelloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameSetNotFound { game_type, set_id } => {
                write!(f, "{} set not found: {}", game_type, set_id)
            }
            Self::ValidationFailed { field, message } => write!(f, "Validation failed on '{}': {}", field, message),
            Self::StorageError { message } => write!(f, "Storage error: {}", message),
            Self::ExternalServiceError { service, message } => write!(f, "External service '{}' error: {}", service, message),
            Self::NotFound => write!(f, "Resource not found"),
            Self::Forbidden => write!(f, "Access forbidden"),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

impl std::error::Error for IntelloError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_set_not_found_error() {
        let err = IntelloError::game_not_found("keywords", "kw-456");
        assert_eq!(err.code(), ErrorCode::IntelloGameSetNotFound);
        assert_eq!(err.status_code(), 404);
        assert!(err.to_string().contains("keywords"));
        assert!(err.to_string().contains("kw-456"));
    }

    #[test]
    fn test_validation_failed_error() {
        let err = IntelloError::validation("name", "Name cannot be empty");
        assert_eq!(err.code(), ErrorCode::IntelloValidationFailed);
        assert_eq!(err.status_code(), 400);
        assert!(err.to_string().contains("name"));
    }

    #[test]
    fn test_storage_error() {
        let err = IntelloError::storage("Failed to write file");
        assert_eq!(err.code(), ErrorCode::IntelloStorageError);
        assert_eq!(err.status_code(), 500);
        assert!(err.to_string().contains("Failed to write file"));
    }
}
