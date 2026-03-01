//! Global error - Top-level application error with ResponseError impl
//!
//! This is the main error type that all service errors convert to.
//! It implements actix-web's ResponseError for HTTP responses.

use crate::http_api::utils::{InternalError, ValidationError};
use crate::infra::session::SessionError;
use crate::infra::supabase::SupabaseError;
use crate::services::app_registry::error_domain::AppError as AppsError;
use crate::services::auth::error_domain::AuthError;
use crate::services::collection::error_domain::CollectionError;
use crate::services::intello::error_domain::IntelloError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use tracing::{error, warn};

// == ERROR RESPONSE STRUCT ==

/// JSON response body for errors
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: &'static str,
    pub message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<&'static str>,
}

// == MAIN ERROR TYPE ==

/// Main application error type - all errors bubble up to this
#[derive(Debug)]
pub enum AppError {
    /// Authentication errors
    Auth(AuthError),
    /// Validation errors
    Validation(ValidationError),
    /// Collection errors
    Collection(CollectionError),
    /// Intello errors
    Intello(IntelloError),
    /// App management errors
    App(AppsError),
    /// Session errors
    Session(SessionError),
    /// Internal error
    Internal(InternalError),
}

impl AppError {
    /// Get the error code string
    pub fn code(&self) -> &'static str {
        match self {
            Self::Auth(err) => err.code(),
            Self::Validation(err) => err.code(),
            Self::Collection(err) => err.code(),
            Self::Intello(err) => err.code(),
            Self::App(err) => err.code(),
            Self::Session(err) => err.code(),
            Self::Internal(err) => err.code(),
        }
    }

    /// Get the user-friendly message
    pub fn message(&self) -> &'static str {
        match self {
            Self::Auth(err) => err.message(),
            Self::Validation(err) => err.message(),
            Self::Collection(err) => err.message(),
            Self::Intello(err) => err.message(),
            Self::App(err) => err.message(),
            Self::Session(err) => err.message(),
            Self::Internal(err) => err.message(),
        }
    }

    /// Helper to create validation error
    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::Validation(ValidationError::new(field, message))
    }

    /// Automatic logging based on error type
    fn log(&self) {
        match self {
            Self::Auth(err) => {
                warn!(error_code = %self.code(), error = %err, "Authentication error");
            }
            Self::Validation(err) => {
                warn!(error_code = %self.code(), field = %err.field, message = %err.message, "Validation error");
            }
            Self::Collection(err) => {
                warn!(error_code = %self.code(), error = %err, "Collection error");
            }
            Self::Intello(err) => {
                warn!(error_code = %self.code(), error = %err, "Intello error");
            }
            Self::App(err) => {
                warn!(error_code = %self.code(), error = %err, "App error");
            }
            Self::Session(err) => {
                warn!(error_code = %self.code(), error = %err, "Session error");
            }
            Self::Internal(err) => {
                error!(error_code = %self.code(), message = %err.0, "Internal error");
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auth(err) => write!(fmt, "{}", err),
            Self::Validation(err) => write!(fmt, "{}", err),
            Self::Collection(err) => write!(fmt, "{}", err),
            Self::Intello(err) => write!(fmt, "{}", err),
            Self::App(err) => write!(fmt, "{}", err),
            Self::Session(err) => write!(fmt, "{}", err),
            Self::Internal(err) => write!(fmt, "{}", err),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Auth(err) => Some(err),
            Self::Validation(err) => Some(err),
            Self::Collection(err) => Some(err),
            Self::Intello(err) => Some(err),
            Self::App(err) => Some(err),
            Self::Session(err) => Some(err),
            Self::Internal(err) => Some(err),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Auth(err) => err.status(),
            Self::Validation(err) => err.status(),
            Self::Collection(err) => err.status(),
            Self::Intello(err) => err.status(),
            Self::App(err) => err.status(),
            Self::Session(err) => err.status(),
            Self::Internal(err) => err.status(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        self.log();
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            code: self.code(),
            message: self.message(),
            field: match self {
                Self::Validation(err) => Some(err.field),
                _ => None,
            },
        })
    }
}

/// Convenient Result type for handlers
pub type AppResult<T> = Result<T, AppError>;

// == CONVERSIONS ==

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl From<SupabaseError> for AppError {
    fn from(err: SupabaseError) -> Self {
        Self::Auth(AuthError::from(err))
    }
}

impl From<CollectionError> for AppError {
    fn from(err: CollectionError) -> Self {
        Self::Collection(err)
    }
}

impl From<IntelloError> for AppError {
    fn from(err: IntelloError) -> Self {
        Self::Intello(err)
    }
}

impl From<AppsError> for AppError {
    fn from(err: AppsError) -> Self {
        Self::App(err)
    }
}

impl From<SessionError> for AppError {
    fn from(err: SessionError) -> Self {
        Self::Session(err)
    }
}

impl From<ValidationError> for AppError {
    fn from(err: ValidationError) -> Self {
        Self::Validation(err)
    }
}

impl From<InternalError> for AppError {
    fn from(err: InternalError) -> Self {
        Self::Internal(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- code() delegation tests --

    #[test]
    fn test_code_delegates_to_intello() {
        // arrange
        let err = AppError::Intello(
            IntelloError::NotFound,
        );

        // act / assert
        assert_eq!(err.code(), "INTELLO_NOT_FOUND");
    }

    #[test]
    fn test_code_delegates_to_collection() {
        // arrange
        let err = AppError::Collection(
            CollectionError::DvdNotFound {
                dvd_id: "d1".into(),
            },
        );

        // act / assert
        assert_eq!(err.code(), "COLLECTION_DVD_NOT_FOUND");
    }

    #[test]
    fn test_code_delegates_to_auth() {
        // arrange
        let err =
            AppError::Auth(AuthError::InvalidCredentials);

        // act / assert
        assert_eq!(err.code(), "AUTH_INVALID_CREDENTIALS");
    }

    #[test]
    fn test_code_delegates_to_session() {
        // arrange
        let err =
            AppError::Session(SessionError::NotFound);

        // act / assert
        assert_eq!(err.code(), "SESSION_NOT_FOUND");
    }

    // -- message() delegation tests --

    #[test]
    fn test_message_delegates_to_inner() {
        // arrange
        let err = AppError::Intello(
            IntelloError::Forbidden,
        );

        // act / assert
        assert_eq!(err.message(), "Access forbidden");
    }

    // -- validation() helper test --

    #[test]
    fn test_validation_creates_validation_error() {
        // arrange / act
        let err =
            AppError::validation("email", "is required");

        // assert
        assert_eq!(err.code(), "VALIDATION_FAILED");
        assert_eq!(err.message(), "Invalid input data");
    }

    // -- From conversion tests --

    #[test]
    fn test_from_auth_error() {
        // arrange
        let auth = AuthError::NotAllowed;

        // act
        let err: AppError = auth.into();

        // assert
        assert!(matches!(err, AppError::Auth(_)));
        assert_eq!(err.code(), "AUTH_NOT_ALLOWED");
    }

    #[test]
    fn test_from_collection_error() {
        // arrange
        let col = CollectionError::DvdNotFound {
            dvd_id: "x".into(),
        };

        // act
        let err: AppError = col.into();

        // assert
        assert!(matches!(err, AppError::Collection(_)));
    }

    #[test]
    fn test_from_intello_error() {
        // arrange
        let intello = IntelloError::Forbidden;

        // act
        let err: AppError = intello.into();

        // assert
        assert!(matches!(err, AppError::Intello(_)));
    }

    #[test]
    fn test_from_apps_error() {
        // arrange
        let app =
            AppsError::NotFound("intello".into());

        // act
        let err: AppError = app.into();

        // assert
        assert!(matches!(err, AppError::App(_)));
    }

    #[test]
    fn test_from_session_error() {
        // arrange
        let session = SessionError::Expired;

        // act
        let err: AppError = session.into();

        // assert
        assert!(matches!(err, AppError::Session(_)));
    }

    #[test]
    fn test_from_validation_error() {
        // arrange
        let val = ValidationError::new("f", "msg");

        // act
        let err: AppError = val.into();

        // assert
        assert!(matches!(err, AppError::Validation(_)));
    }

    #[test]
    fn test_from_internal_error() {
        // arrange
        let internal = InternalError::new("boom");

        // act
        let err: AppError = internal.into();

        // assert
        assert!(matches!(err, AppError::Internal(_)));
    }

    // -- Display delegation --

    #[test]
    fn test_display_delegates_to_inner() {
        // arrange
        let err = AppError::Intello(
            IntelloError::NotFound,
        );

        // act / assert
        assert_eq!(
            err.to_string(),
            "Resource not found"
        );
    }

    // -- ResponseError status_code --

    #[test]
    fn test_status_code_delegates_to_inner() {
        // arrange
        let err =
            AppError::Auth(AuthError::InvalidCredentials);

        // act / assert
        assert_eq!(
            err.status_code(),
            StatusCode::UNAUTHORIZED
        );
    }
}
