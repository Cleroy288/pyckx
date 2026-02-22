//! Intello error - Service layer errors for Intello app operations
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
use std::fmt;

// ** IntelloError **
// ==> Service layer errors for Intello app operations
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
pub enum IntelloError {
    GameSetNotFound { game_type: String, set_id: String },
    ValidationFailed { field: String, message: String },
    StorageError { message: String },
    ExternalServiceError { service: String, message: String },
    NotFound,
    Forbidden,
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
            Self::ExternalServiceError { .. } => {
                "INTELLO_EXTERNAL_SERVICE_ERROR"
            }
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

    // ** game_not_found **
    // ==> Creates a GameSetNotFound error for any game type
    //
    // @ game_type : The type of game (e.g., "qcm", "flashcard")
    // @ set_id : The set ID that was not found
    // @ returns : IntelloError::GameSetNotFound variant
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
    // @ returns : IntelloError::ValidationFailed variant
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
    // @ returns : IntelloError::StorageError variant
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
    // @ returns : IntelloError::ExternalServiceError variant
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

impl fmt::Display for IntelloError {
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

impl std::error::Error for IntelloError {}

#[cfg(test)]
mod tests {
    use super::*;

    // -- code() tests --

    #[test]
    fn test_code_game_set_not_found_returns_correct_code() {
        // arrange
        let err = IntelloError::game_not_found("qcm", "abc");

        // act
        let code = err.code();

        // assert
        assert_eq!(code, "INTELLO_GAME_SET_NOT_FOUND");
    }

    #[test]
    fn test_code_validation_failed_returns_correct_code() {
        // arrange
        let err = IntelloError::validation("name", "too short");

        // act
        let code = err.code();

        // assert
        assert_eq!(code, "INTELLO_VALIDATION_FAILED");
    }

    #[test]
    fn test_code_storage_error_returns_correct_code() {
        // arrange
        let err = IntelloError::storage("db down");

        // act
        let code = err.code();

        // assert
        assert_eq!(code, "INTELLO_STORAGE_ERROR");
    }

    #[test]
    fn test_code_external_service_returns_correct_code() {
        // arrange
        let err = IntelloError::external("openai", "timeout");

        // act
        let code = err.code();

        // assert
        assert_eq!(code, "INTELLO_EXTERNAL_SERVICE_ERROR");
    }

    #[test]
    fn test_code_not_found_returns_correct_code() {
        // arrange
        let err = IntelloError::NotFound;

        // act / assert
        assert_eq!(err.code(), "INTELLO_NOT_FOUND");
    }

    #[test]
    fn test_code_forbidden_returns_correct_code() {
        // arrange
        let err = IntelloError::Forbidden;

        // act / assert
        assert_eq!(err.code(), "INTELLO_FORBIDDEN");
    }

    #[test]
    fn test_code_conflict_returns_correct_code() {
        // arrange
        let err = IntelloError::Conflict("dup".into());

        // act / assert
        assert_eq!(err.code(), "INTELLO_CONFLICT");
    }

    // -- message() tests --

    #[test]
    fn test_message_scenarios() {
        let cases = vec![
            (
                IntelloError::game_not_found("qcm", "1"),
                "Game set not found",
            ),
            (
                IntelloError::validation("f", "m"),
                "Invalid QCM set data",
            ),
            (
                IntelloError::storage("err"),
                "Failed to access Intello storage",
            ),
            (
                IntelloError::external("s", "m"),
                "External service error",
            ),
            (IntelloError::NotFound, "Resource not found"),
            (IntelloError::Forbidden, "Access forbidden"),
            (
                IntelloError::Conflict("x".into()),
                "Resource conflict",
            ),
        ];
        for (err, expected) in cases {
            assert_eq!(
                err.message(),
                expected,
                "message() for {:?}",
                err
            );
        }
    }

    // -- status() tests --

    #[test]
    fn test_status_scenarios() {
        let cases = vec![
            (
                IntelloError::game_not_found("q", "1"),
                StatusCode::NOT_FOUND,
            ),
            (
                IntelloError::validation("f", "m"),
                StatusCode::BAD_REQUEST,
            ),
            (
                IntelloError::storage("e"),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            (
                IntelloError::external("s", "m"),
                StatusCode::BAD_GATEWAY,
            ),
            (IntelloError::NotFound, StatusCode::NOT_FOUND),
            (IntelloError::Forbidden, StatusCode::FORBIDDEN),
            (
                IntelloError::Conflict("x".into()),
                StatusCode::CONFLICT,
            ),
        ];
        for (err, expected) in cases {
            assert_eq!(
                err.status(),
                expected,
                "status() for {:?}",
                err
            );
        }
    }

    // -- constructor tests --

    #[test]
    fn test_game_not_found_stores_fields() {
        // arrange / act
        let err = IntelloError::game_not_found("flash", "id-42");

        // assert
        match err {
            IntelloError::GameSetNotFound {
                game_type,
                set_id,
            } => {
                assert_eq!(game_type, "flash");
                assert_eq!(set_id, "id-42");
            }
            _ => panic!("Expected GameSetNotFound"),
        }
    }

    #[test]
    fn test_validation_stores_fields() {
        // arrange / act
        let err = IntelloError::validation("email", "invalid");

        // assert
        match err {
            IntelloError::ValidationFailed { field, message } => {
                assert_eq!(field, "email");
                assert_eq!(message, "invalid");
            }
            _ => panic!("Expected ValidationFailed"),
        }
    }

    #[test]
    fn test_storage_stores_message() {
        // arrange / act
        let err = IntelloError::storage("disk full");

        // assert
        match err {
            IntelloError::StorageError { message } => {
                assert_eq!(message, "disk full");
            }
            _ => panic!("Expected StorageError"),
        }
    }

    #[test]
    fn test_external_stores_fields() {
        // arrange / act
        let err = IntelloError::external("openai", "rate limit");

        // assert
        match err {
            IntelloError::ExternalServiceError {
                service,
                message,
            } => {
                assert_eq!(service, "openai");
                assert_eq!(message, "rate limit");
            }
            _ => panic!("Expected ExternalServiceError"),
        }
    }

    // -- Display tests --

    #[test]
    fn test_display_game_set_not_found() {
        // arrange
        let err = IntelloError::game_not_found("qcm", "set-1");

        // act
        let display = err.to_string();

        // assert
        assert_eq!(display, "qcm set not found: set-1");
    }

    #[test]
    fn test_display_validation_failed() {
        // arrange
        let err = IntelloError::validation("name", "too short");

        // act / assert
        assert_eq!(
            err.to_string(),
            "Validation failed on 'name': too short"
        );
    }

    #[test]
    fn test_display_not_found() {
        assert_eq!(
            IntelloError::NotFound.to_string(),
            "Resource not found"
        );
    }

    #[test]
    fn test_display_forbidden() {
        assert_eq!(
            IntelloError::Forbidden.to_string(),
            "Access forbidden"
        );
    }

    #[test]
    fn test_display_conflict() {
        // arrange
        let err = IntelloError::Conflict("duplicate".into());

        // act / assert
        assert_eq!(err.to_string(), "Conflict: duplicate");
    }
}
