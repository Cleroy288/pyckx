//! Auth error - Service layer errors

use crate::error::shared::ErrorCode;
use crate::error::supabase::SupabaseError;
use actix_web::http::StatusCode;
use std::fmt;

/// Authentication-related errors
#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    #[allow(dead_code)] // Used by registration feature (currently disabled)
    NotAllowed,
    External(SupabaseError),
}

impl AuthError {
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::InvalidCredentials => ErrorCode::InvalidCredentials,
            Self::NotAllowed => ErrorCode::NotAllowed,
            Self::External(e) => e.code(),
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
