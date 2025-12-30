//! App-specific errors
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
use std::fmt;

/// App-specific error type
#[derive(Debug)]
pub enum AppError {
    /// App not found in registry or database
    NotFound(String),
    /// App already exists
    AlreadyExists(String),
    /// User already has this app
    UserAppAlreadyAdded { user_id: String, app_name: String },
    /// User doesn't have this app
    UserAppNotFound { user_id: String, app_name: String },
}

impl AppError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "APP_NOT_FOUND",
            Self::AlreadyExists(_) => "APP_ALREADY_EXISTS",
            Self::UserAppAlreadyAdded { .. } => "USER_APP_ALREADY_ADDED",
            Self::UserAppNotFound { .. } => "USER_APP_NOT_FOUND",
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "App not found",
            Self::AlreadyExists(_) => "App already exists",
            Self::UserAppAlreadyAdded { .. } => "User already has this app",
            Self::UserAppNotFound { .. } => "User doesn't have this app",
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AlreadyExists(_) => StatusCode::CONFLICT,
            Self::UserAppAlreadyAdded { .. } => StatusCode::CONFLICT,
            Self::UserAppNotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "App '{}' not found", name),
            Self::AlreadyExists(name) => write!(f, "App '{}' already exists", name),
            Self::UserAppAlreadyAdded { user_id, app_name } => {
                write!(f, "User '{}' already has app '{}'", user_id, app_name)
            }
            Self::UserAppNotFound { user_id, app_name } => {
                write!(f, "User '{}' doesn't have app '{}'", user_id, app_name)
            }
        }
    }
}

impl std::error::Error for AppError {}
