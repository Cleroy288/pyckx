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
            Self::NotAllowed => {
                write!(f, "User not authorized to access this platform")
            }
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
                if *status == StatusCode::UNAUTHORIZED
                    || *status == StatusCode::BAD_REQUEST =>
            {
                Self::InvalidCredentials
            }
            _ => Self::External(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- code() tests --

    #[test]
    fn test_code_invalid_credentials() {
        // arrange
        let err = AuthError::InvalidCredentials;

        // act / assert
        assert_eq!(err.code(), "AUTH_INVALID_CREDENTIALS");
    }

    #[test]
    fn test_code_not_allowed() {
        // arrange
        let err = AuthError::NotAllowed;

        // act / assert
        assert_eq!(err.code(), "AUTH_NOT_ALLOWED");
    }

    // -- message() tests --

    #[test]
    fn test_message_invalid_credentials() {
        // arrange
        let err = AuthError::InvalidCredentials;

        // act / assert
        assert_eq!(err.message(), "Invalid email or password");
    }

    #[test]
    fn test_message_not_allowed() {
        // arrange
        let err = AuthError::NotAllowed;

        // act / assert
        assert_eq!(
            err.message(),
            "User not authorized to access this platform"
        );
    }

    // -- status() tests --

    #[test]
    fn test_status_invalid_credentials() {
        // arrange
        let err = AuthError::InvalidCredentials;

        // act / assert
        assert_eq!(err.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_status_not_allowed() {
        // arrange
        let err = AuthError::NotAllowed;

        // act / assert
        assert_eq!(err.status(), StatusCode::FORBIDDEN);
    }

    // -- Display tests --

    #[test]
    fn test_display_invalid_credentials() {
        assert_eq!(
            AuthError::InvalidCredentials.to_string(),
            "Invalid credentials"
        );
    }

    #[test]
    fn test_display_not_allowed() {
        assert_eq!(
            AuthError::NotAllowed.to_string(),
            "User not authorized to access this platform"
        );
    }

    // -- From<SupabaseError> tests --

    #[test]
    fn test_from_supabase_unauthorized_maps_to_invalid() {
        // arrange
        let supa_err = SupabaseError::Http {
            status: StatusCode::UNAUTHORIZED,
            body: "bad".to_string(),
        };

        // act
        let auth_err = AuthError::from(supa_err);

        // assert
        assert!(matches!(
            auth_err,
            AuthError::InvalidCredentials
        ));
    }

    #[test]
    fn test_from_supabase_bad_request_maps_to_invalid() {
        // arrange
        let supa_err = SupabaseError::Http {
            status: StatusCode::BAD_REQUEST,
            body: "err".to_string(),
        };

        // act
        let auth_err = AuthError::from(supa_err);

        // assert
        assert!(matches!(
            auth_err,
            AuthError::InvalidCredentials
        ));
    }

    #[test]
    fn test_from_supabase_other_http_maps_to_external() {
        // arrange
        let supa_err = SupabaseError::Http {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: "500".to_string(),
        };

        // act
        let auth_err = AuthError::from(supa_err);

        // assert
        assert!(matches!(auth_err, AuthError::External(_)));
    }

    #[test]
    fn test_from_supabase_parse_maps_to_external() {
        // arrange
        let supa_err = SupabaseError::Parse {
            body: "bad json".to_string(),
        };

        // act
        let auth_err = AuthError::from(supa_err);

        // assert
        assert!(matches!(auth_err, AuthError::External(_)));
    }

    // -- Error::source tests --

    #[test]
    fn test_source_invalid_credentials_is_none() {
        // arrange
        let err = AuthError::InvalidCredentials;

        // act / assert
        assert!(
            std::error::Error::source(&err).is_none()
        );
    }

    #[test]
    fn test_source_not_allowed_is_none() {
        // arrange
        let err = AuthError::NotAllowed;

        // act / assert
        assert!(
            std::error::Error::source(&err).is_none()
        );
    }
}
