//! Global error - Top-level application error
//!
//! Main error type that all service errors convert to.
//! Implements actix-web ResponseError for HTTP responses.

use crate::configs::ConfigError;
use crate::http_api::utils::{
    InternalError, ValidationError,
};
use crate::infra::session::SessionError;
use crate::infra::supabase::SupabaseAuthError;
use crate::services::app_registry::error_domain::AppError as AppsError;
use crate::services::auth::error_domain::AuthError;
use crate::services::collection::error_domain::CollectionError;
use crate::services::error_domain::StudyError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use tracing::{error, warn};

/// JSON response body for errors
#[derive(Serialize)]
pub struct ErrorResponse {
    /// Error code identifier
    pub code: &'static str,
    /// User-friendly message
    pub message: &'static str,
    /// Field that caused the error (validation only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<&'static str>,
}

/// Main application error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// Authentication errors
    #[error("{0}")]
    Auth(AuthError),
    /// Validation errors
    #[error("{0}")]
    Validation(ValidationError),
    /// Collection errors
    #[error("{0}")]
    Collection(CollectionError),
    /// Study errors
    #[error("{0}")]
    Study(StudyError),
    /// App management errors
    #[error("{0}")]
    App(AppsError),
    /// Session errors
    #[error("{0}")]
    Session(SessionError),
    /// Internal error
    #[error("{0}")]
    Internal(InternalError),
    /// Configuration error
    #[error("{0}")]
    Config(ConfigError),
}

impl AppError {
    /// Get the error code string
    pub fn code(&self) -> &'static str {
        match self {
            Self::Auth(e) => e.code(),
            Self::Validation(e) => e.code(),
            Self::Collection(e) => e.code(),
            Self::Study(e) => e.code(),
            Self::App(e) => e.code(),
            Self::Session(e) => e.code(),
            Self::Internal(e) => e.code(),
            Self::Config(_) => "CONFIG_ERROR",
        }
    }

    /// Get the user-friendly message
    pub fn message(&self) -> &'static str {
        match self {
            Self::Auth(e) => e.message(),
            Self::Validation(e) => e.message(),
            Self::Collection(e) => e.message(),
            Self::Study(e) => e.message(),
            Self::App(e) => e.message(),
            Self::Session(e) => e.message(),
            Self::Internal(e) => e.message(),
            Self::Config(_) => "Configuration error",
        }
    }

    /// Helper to create validation error
    pub fn validation(
        field: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self::Validation(
            ValidationError::new(field, message),
        )
    }

    /// Automatic logging based on error type
    fn log(&self) {
        match self {
            Self::Auth(e) => {
                warn!(
                    error_code = %self.code(),
                    error = %e,
                    "Authentication error"
                );
            }
            Self::Validation(e) => {
                warn!(
                    error_code = %self.code(),
                    field = %e.field,
                    message = %e.message,
                    "Validation error"
                );
            }
            Self::Internal(e) => {
                error!(
                    error_code = %self.code(),
                    message = %e.0,
                    "Internal error"
                );
            }
            Self::Config(e) => {
                error!(
                    error_code = %self.code(),
                    error = %e,
                    "Config error"
                );
            }
            other => {
                warn!(
                    error_code = %other.code(),
                    error = %other,
                    "Application error"
                );
            }
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Auth(e) => e.status(),
            Self::Validation(e) => e.status(),
            Self::Collection(e) => e.status(),
            Self::Study(e) => e.status(),
            Self::App(e) => e.status(),
            Self::Session(e) => e.status(),
            Self::Internal(e) => e.status(),
            Self::Config(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        self.log();
        let field = match self {
            Self::Validation(e) => Some(e.field),
            _ => None,
        };
        HttpResponse::build(self.status_code())
            .json(ErrorResponse {
                code: self.code(),
                message: self.message(),
                field,
            })
    }
}

/// Convenient Result type for handlers
pub type AppResult<T> = Result<T, AppError>;

impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        Self::Auth(e)
    }
}

impl From<SupabaseAuthError> for AppError {
    fn from(e: SupabaseAuthError) -> Self {
        Self::Auth(AuthError::from(e))
    }
}

impl From<CollectionError> for AppError {
    fn from(e: CollectionError) -> Self {
        Self::Collection(e)
    }
}

impl From<StudyError> for AppError {
    fn from(e: StudyError) -> Self {
        Self::Study(e)
    }
}

impl From<AppsError> for AppError {
    fn from(e: AppsError) -> Self {
        Self::App(e)
    }
}

impl From<SessionError> for AppError {
    fn from(e: SessionError) -> Self {
        Self::Session(e)
    }
}

impl From<ValidationError> for AppError {
    fn from(e: ValidationError) -> Self {
        Self::Validation(e)
    }
}

impl From<InternalError> for AppError {
    fn from(e: InternalError) -> Self {
        Self::Internal(e)
    }
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        Self::Config(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_delegation() {
        let cases: Vec<(AppError, &str)> = vec![
            (
                AppError::Study(StudyError::NotFound),
                "STUDY_NOT_FOUND",
            ),
            (
                AppError::Collection(
                    CollectionError::DvdNotFound {
                        dvd_id: "d1".into(),
                    },
                ),
                "COLLECTION_DVD_NOT_FOUND",
            ),
            (
                AppError::Auth(
                    AuthError::InvalidCredentials,
                ),
                "AUTH_INVALID_CREDENTIALS",
            ),
            (
                AppError::Session(SessionError::NotFound),
                "SESSION_NOT_FOUND",
            ),
            (
                AppError::Config(
                    ConfigError::MissingEnvVar {
                        var_name: "X",
                    },
                ),
                "CONFIG_ERROR",
            ),
        ];
        for (err, expected) in cases {
            assert_eq!(
                err.code(),
                expected,
                "code() for {:?}",
                err
            );
        }
    }

    #[test]
    fn test_from_conversions() {
        let auth: AppError =
            AuthError::NotAllowed.into();
        assert!(matches!(auth, AppError::Auth(_)));

        let col: AppError =
            CollectionError::DvdNotFound {
                dvd_id: "x".into(),
            }
            .into();
        assert!(matches!(col, AppError::Collection(_)));

        let study_err: AppError =
            StudyError::Forbidden.into();
        assert!(matches!(study_err, AppError::Study(_)));

        let app: AppError =
            AppsError::NotFound("i".into()).into();
        assert!(matches!(app, AppError::App(_)));

        let session: AppError =
            SessionError::Expired.into();
        assert!(matches!(session, AppError::Session(_)));

        let val: AppError =
            ValidationError::new("f", "msg").into();
        assert!(matches!(val, AppError::Validation(_)));

        let internal: AppError =
            InternalError::new("boom").into();
        assert!(matches!(
            internal,
            AppError::Internal(_)
        ));

        let cfg: AppError =
            ConfigError::MissingEnvVar { var_name: "X" }
                .into();
        assert!(matches!(cfg, AppError::Config(_)));
    }

    #[test]
    fn test_display_delegates_to_inner() {
        let err =
            AppError::Study(StudyError::NotFound);
        assert_eq!(
            err.to_string(),
            "Resource not found"
        );
    }

    #[test]
    fn test_status_code_delegates_to_inner() {
        let err = AppError::Auth(
            AuthError::InvalidCredentials,
        );
        assert_eq!(
            err.status_code(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[test]
    fn test_validation_helper() {
        let err =
            AppError::validation("email", "required");
        assert_eq!(err.code(), "VALIDATION_FAILED");
        assert_eq!(err.message(), "Invalid input data");
    }
}
