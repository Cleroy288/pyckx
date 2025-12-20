//! App error - Top-level application error with ResponseError impl

use crate::error::auth::AuthError;
use crate::error::collection::CollectionError;
use crate::error::intello::IntelloError;
use crate::error::shared::{ErrorCode, ErrorResponse};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use tracing::{error, warn};

/// Main application error type - all errors bubble up to this
#[derive(Debug)]
pub enum AppError {
    /// Authentication errors
    Auth(AuthError),
    /// Validation errors
    Validation { field: &'static str, message: String },
    /// Collection errors
    Collection(CollectionError),
    /// Intello errors
    Intello(IntelloError),
    /// Session errors (unauthorized)
    SessionNotFound,
    #[allow(dead_code)]
    SessionExpired,
    /// App not found
    AppNotFound(String),
    /// App already exists
    AppAlreadyExists(String),
    /// User already has this app
    UserAppAlreadyAdded { user_id: String, app_name: String },
    /// User doesn't have this app
    UserAppNotFound { user_id: String, app_name: String },
    /// Internal error
    Internal(String),
}

impl AppError {
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Auth(e) => e.code(),
            Self::Validation { .. } => ErrorCode::ValidationFailed,
            Self::Collection(e) => e.code(),
            Self::Intello(e) => e.code(),
            Self::SessionNotFound => ErrorCode::SessionNotFound,
            Self::SessionExpired => ErrorCode::SessionExpired,
            Self::AppNotFound(_) => ErrorCode::AppNotFound,
            Self::AppAlreadyExists(_) => ErrorCode::AppAlreadyExists,
            Self::UserAppAlreadyAdded { .. } => ErrorCode::UserAppAlreadyAdded,
            Self::UserAppNotFound { .. } => ErrorCode::UserAppNotFound,
            Self::Internal(_) => ErrorCode::InternalError,
        }
    }

    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::Validation { field, message: message.into() }
    }

    /// Automatic logging based on error type
    pub(super) fn log(&self) {
        match self {
            Self::Auth(AuthError::InvalidCredentials) => {
                warn!(error_code = %self.code().as_str(), "Authentication failed: invalid credentials");
            }
            Self::Auth(AuthError::NotAllowed) => {
                warn!(error_code = %self.code().as_str(), "Authentication failed: user not authorized");
            }
            Self::Auth(AuthError::External(e)) => {
                error!(error_code = %self.code().as_str(), supabase_error = %e, "External auth service error");
            }
            Self::Validation { field, message } => {
                warn!(error_code = %self.code().as_str(), field = %field, message = %message, "Validation error");
            }
            Self::Collection(e) => {
                match e {
                    CollectionError::DvdNotFound { dvd_id } => {
                        warn!(error_code = %self.code().as_str(), dvd_id = %dvd_id, "DVD not found");
                    }
                    CollectionError::DvdDuplicate { name } => {
                        warn!(error_code = %self.code().as_str(), name = %name, "Duplicate DVD");
                    }
                    CollectionError::UserNotFound { user_id } => {
                        warn!(error_code = %self.code().as_str(), user_id = %user_id, "User collection not found");
                    }
                    CollectionError::StorageError { message } => {
                        error!(error_code = %self.code().as_str(), message = %message, "Collection storage error");
                    }
                }
            }
            Self::Intello(e) => {
                match e {
                    IntelloError::QcmSetNotFound { set_id } => {
                        warn!(error_code = %self.code().as_str(), set_id = %set_id, "QCM set not found");
                    }
                    IntelloError::GameSetNotFound { game_type, set_id } => {
                        warn!(error_code = %self.code().as_str(), game_type = %game_type, set_id = %set_id, "Game set not found");
                    }
                    IntelloError::ValidationFailed { field, message } => {
                        warn!(error_code = %self.code().as_str(), field = %field, message = %message, "Intello validation error");
                    }
                    IntelloError::StorageError { message } => {
                        error!(error_code = %self.code().as_str(), message = %message, "Intello storage error");
                    }
                    IntelloError::ExternalServiceError { service, message } => {
                        error!(error_code = %self.code().as_str(), service = %service, message = %message, "External service error");
                    }
                }
            }
            Self::SessionNotFound => {
                warn!(error_code = %self.code().as_str(), "Session not found");
            }
            Self::SessionExpired => {
                warn!(error_code = %self.code().as_str(), "Session expired");
            }
            Self::AppNotFound(name) => {
                warn!(error_code = %self.code().as_str(), app_name = %name, "App not found");
            }
            Self::AppAlreadyExists(name) => {
                warn!(error_code = %self.code().as_str(), app_name = %name, "App already exists");
            }
            Self::UserAppAlreadyAdded { user_id, app_name } => {
                warn!(error_code = %self.code().as_str(), user_id = %user_id, app_name = %app_name, "User already has app");
            }
            Self::UserAppNotFound { user_id, app_name } => {
                warn!(error_code = %self.code().as_str(), user_id = %user_id, app_name = %app_name, "User app not found");
            }
            Self::Internal(msg) => {
                error!(error_code = %self.code().as_str(), message = %msg, "Internal error");
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auth(e) => write!(f, "{}", e),
            Self::Validation { field, message } => write!(f, "Validation error on '{}': {}", field, message),
            Self::Collection(e) => write!(f, "{}", e),
            Self::Intello(e) => write!(f, "{}", e),
            Self::SessionNotFound => write!(f, "Session not found"),
            Self::SessionExpired => write!(f, "Session expired"),
            Self::AppNotFound(name) => write!(f, "App '{}' not found", name),
            Self::AppAlreadyExists(name) => write!(f, "App '{}' already exists", name),
            Self::UserAppAlreadyAdded { user_id, app_name } => write!(f, "User '{}' already has app '{}'", user_id, app_name),
            Self::UserAppNotFound { user_id, app_name } => write!(f, "User '{}' doesn't have app '{}'", user_id, app_name),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Auth(e) => Some(e),
            Self::Collection(e) => Some(e),
            Self::Intello(e) => Some(e),
            _ => None,
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.code().status()
    }

    fn error_response(&self) -> HttpResponse {
        self.log();
        let code = self.code();
        HttpResponse::build(code.status()).json(ErrorResponse {
            code: code.as_str(),
            message: code.message(),
            field: match self {
                Self::Validation { field, .. } => Some(*field),
                _ => None,
            },
        })
    }
}

/// Convenient Result type for handlers
pub type AppResult<T> = Result<T, AppError>;
