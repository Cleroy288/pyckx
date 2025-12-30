//! Global error - Top-level application error with ResponseError impl
//!
//! This is the main error type that all service errors convert to.
//! It implements actix-web's ResponseError for HTTP responses.

use crate::services::app_registry::error_domain::AppError as AppsError;
use crate::services::intello::error_domain::IntelloError;
use crate::infra::session::SessionError;
use crate::infra::supabase::SupabaseError;
use crate::services::collection::error_domain::CollectionError;
use crate::services::auth::error_domain::AuthError;
use crate::http_api::utils::{InternalError, ValidationError};
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
            Self::Auth(e) => e.code(),
            Self::Validation(e) => e.code(),
            Self::Collection(e) => e.code(),
            Self::Intello(e) => e.code(),
            Self::App(e) => e.code(),
            Self::Session(e) => e.code(),
            Self::Internal(e) => e.code(),
        }
    }

    /// Get the user-friendly message
    pub fn message(&self) -> &'static str {
        match self {
            Self::Auth(e) => e.message(),
            Self::Validation(e) => e.message(),
            Self::Collection(e) => e.message(),
            Self::Intello(e) => e.message(),
            Self::App(e) => e.message(),
            Self::Session(e) => e.message(),
            Self::Internal(e) => e.message(),
        }
    }

    /// Helper to create validation error
    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::Validation(ValidationError::new(field, message))
    }

    /// Automatic logging based on error type
    fn log(&self) {
        match self {
            Self::Auth(e) => {
                warn!(error_code = %self.code(), error = %e, "Authentication error");
            }
            Self::Validation(e) => {
                warn!(error_code = %self.code(), field = %e.field, message = %e.message, "Validation error");
            }
            Self::Collection(e) => {
                warn!(error_code = %self.code(), error = %e, "Collection error");
            }
            Self::Intello(e) => {
                warn!(error_code = %self.code(), error = %e, "Intello error");
            }
            Self::App(e) => {
                warn!(error_code = %self.code(), error = %e, "App error");
            }
            Self::Session(e) => {
                warn!(error_code = %self.code(), error = %e, "Session error");
            }
            Self::Internal(e) => {
                error!(error_code = %self.code(), message = %e.0, "Internal error");
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auth(e) => write!(f, "{}", e),
            Self::Validation(e) => write!(f, "{}", e),
            Self::Collection(e) => write!(f, "{}", e),
            Self::Intello(e) => write!(f, "{}", e),
            Self::App(e) => write!(f, "{}", e),
            Self::Session(e) => write!(f, "{}", e),
            Self::Internal(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Auth(e) => Some(e),
            Self::Validation(e) => Some(e),
            Self::Collection(e) => Some(e),
            Self::Intello(e) => Some(e),
            Self::App(e) => Some(e),
            Self::Session(e) => Some(e),
            Self::Internal(e) => Some(e),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Auth(e) => e.status(),
            Self::Validation(e) => e.status(),
            Self::Collection(e) => e.status(),
            Self::Intello(e) => e.status(),
            Self::App(e) => e.status(),
            Self::Session(e) => e.status(),
            Self::Internal(e) => e.status(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        self.log();
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            code: self.code(),
            message: self.message(),
            field: match self {
                Self::Validation(e) => Some(e.field),
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
