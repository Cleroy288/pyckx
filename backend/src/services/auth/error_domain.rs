//! Auth error - Service layer errors for authentication
//!
//! Self-contained error with code, message, and status.

use crate::infra::supabase::SupabaseError;
use actix_web::http::StatusCode;
use std::fmt;

/// Authentication-related errors
#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    #[allow(dead_code)]
    NotAllowed,
    External(SupabaseError),
}

impl AuthError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => "AUTH_INVALID_CREDENTIALS",
            Self::NotAllowed => "AUTH_NOT_ALLOWED",
            Self::External(e) => e.code(),
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => "Invalid email or password",
            Self::NotAllowed => "User not authorized to access this platform",
            Self::External(e) => e.message(),
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::NotAllowed => StatusCode::FORBIDDEN,
            Self::External(e) => e.status(),
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCredentials => write!(f, "Invalid credentials"),
            Self::NotAllowed => write!(f, "User not authorized to access this platform"),
            Self::External(e) => write!(f, "External auth error: {}", e),
        }
    }
}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::External(e) => Some(e),
            Self::InvalidCredentials | Self::NotAllowed => None,
        }
    }
}

impl From<SupabaseError> for AuthError {
    fn from(err: SupabaseError) -> Self {
        match &err {
            SupabaseError::Http { status, .. }
                if *status == StatusCode::UNAUTHORIZED || *status == StatusCode::BAD_REQUEST =>
            {
                Self::InvalidCredentials
            }
            _ => Self::External(err),
        }
    }
}
