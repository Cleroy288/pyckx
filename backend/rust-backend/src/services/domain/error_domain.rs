//! Study error - Service layer errors for Study app operations
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
use std::fmt;

// ** StudyError **
// ==> Service layer errors for Study app operations
//
// @ GameSetNotFound : Game set not found for given type and ID
// @ ValidationFailed : Input validation failed for specific field
// @ StorageError : Database or repository operation failed
// @ ExternalServiceError : External API or service call failed
// @ NotFound : Generic resource not found
// @ Forbidden : Access denied, ownership violation
// @ Conflict : Resource conflict, duplicate exists
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum StudyError {
    GameSetNotFound { game_type: String, set_id: String },
    ValidationFailed { field: String, message: String },
    StorageError { message: String },
    ExternalServiceError { service: String, message: String },
    NotFound,
    Forbidden,
    Conflict(String),
}

#[allow(dead_code)]
impl StudyError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::GameSetNotFound { .. } => "STUDY_GAME_SET_NOT_FOUND",
            Self::ValidationFailed { .. } => "STUDY_VALIDATION_FAILED",
            Self::StorageError { .. } => "STUDY_STORAGE_ERROR",
            Self::ExternalServiceError { .. } => {
                "STUDY_EXTERNAL_SERVICE_ERROR"
            }
            Self::NotFound => "STUDY_NOT_FOUND",
            Self::Forbidden => "STUDY_FORBIDDEN",
            Self::Conflict(_) => "STUDY_CONFLICT",
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

    // ** game_not_found **
    // ==> Creates a GameSetNotFound error for any game type
    //
    // @ game_type : The type of game (e.g., "qcm", "flashcard")
    // @ set_id : The set ID that was not found
    // @ returns : StudyError::GameSetNotFound variant
    pub fn game_not_found(
        game_type: impl Into<String>,
        set_id: impl Into<String>,
    ) -> Self {
        Self::GameSetNotFound {
            game_type: game_type.into(),
            set_id: set_id.into(),
        }
    }

    // ** validation **
    // ==> Creates a ValidationFailed error
    //
    // @ field : The field that failed validation
    // @ message : Validation error message
    // @ returns : StudyError::ValidationFailed variant
    pub fn validation(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::ValidationFailed {
            field: field.into(),
            message: message.into(),
        }
    }

    // ** storage **
    // ==> Creates a StorageError
    //
    // @ message : Error message describing the storage failure
    // @ returns : StudyError::StorageError variant
    pub fn storage(message: impl Into<String>) -> Self {
        Self::StorageError {
            message: message.into(),
        }
    }

    // ** external **
    // ==> Creates an ExternalServiceError
    //
    // @ service : Name of the external service that failed
    // @ message : Error message from the external service
    // @ returns : StudyError::ExternalServiceError variant
    pub fn external(
        service: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::ExternalServiceError {
            service: service.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for StudyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameSetNotFound { game_type, set_id } => {
                write!(f, "{} set not found: {}", game_type, set_id)
            }
            Self::ValidationFailed { field, message } => {
                write!(f, "Validation failed on '{}': {}", field, message)
            }
            Self::StorageError { message } => {
                write!(f, "Storage error: {}", message)
            }
            Self::ExternalServiceError { service, message } => {
                write!(f, "External service '{}' error: {}", service, message)
            }
            Self::NotFound => write!(f, "Resource not found"),
            Self::Forbidden => write!(f, "Access forbidden"),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
        }
    }
}

impl std::error::Error for StudyError {}
