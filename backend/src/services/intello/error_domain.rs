//! Intello error - Service layer errors for Intello app operations
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
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
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::GameSetNotFound { .. } => "INTELLO_GAME_SET_NOT_FOUND",
            Self::ValidationFailed { .. } => "INTELLO_VALIDATION_FAILED",
            Self::StorageError { .. } => "INTELLO_STORAGE_ERROR",
            Self::ExternalServiceError { .. } => "INTELLO_EXTERNAL_SERVICE_ERROR",
            Self::NotFound => "INTELLO_NOT_FOUND",
            Self::Forbidden => "INTELLO_FORBIDDEN",
            Self::Conflict(_) => "INTELLO_CONFLICT",
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::GameSetNotFound { .. } => "Game set not found",
            Self::ValidationFailed { .. } => "Invalid QCM set data",
            Self::StorageError { .. } => "Failed to access Intello storage",
            Self::ExternalServiceError { .. } => "External service error",
            Self::NotFound => "Resource not found",
            Self::Forbidden => "Access forbidden",
            Self::Conflict(_) => "Resource conflict",
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::GameSetNotFound { .. } => StatusCode::NOT_FOUND,
            Self::ValidationFailed { .. } => StatusCode::BAD_REQUEST,
            Self::StorageError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ExternalServiceError { .. } => StatusCode::BAD_GATEWAY,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
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


