//! Auth error - Service layer errors for authentication
//!
//! Self-contained error with code, message, and status.

use crate::infra::supabase::SupabaseAuthError;
use actix_web::http::StatusCode;

/// Authentication-related errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// Invalid email or password
    #[error("Invalid credentials")]
    InvalidCredentials,
    /// User not allowed on this platform
    #[allow(dead_code)]
    #[error(
        "User not authorized to access this platform"
    )]
    NotAllowed,
    /// Upstream Supabase auth error
    #[error("External auth error: {0}")]
    External(SupabaseAuthError),
}

impl AuthError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => {
                "AUTH_INVALID_CREDENTIALS"
            }
            Self::NotAllowed => "AUTH_NOT_ALLOWED",
            Self::External(e) => e.code(),
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => {
                "Invalid email or password"
            }
            Self::NotAllowed => {
                "User not authorized to access \
                 this platform"
            }
            Self::External(e) => e.message(),
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => {
                StatusCode::UNAUTHORIZED
            }
            Self::NotAllowed => StatusCode::FORBIDDEN,
            Self::External(e) => e.status(),
        }
    }
}

impl From<SupabaseAuthError> for AuthError {
    fn from(err: SupabaseAuthError) -> Self {
        match &err {
            SupabaseAuthError::Http { status, .. }
                if *status == StatusCode::UNAUTHORIZED
                    || *status
                        == StatusCode::BAD_REQUEST =>
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

    #[test]
    fn test_code_scenarios() {
        let cases = vec![
            (
                AuthError::InvalidCredentials,
                "AUTH_INVALID_CREDENTIALS",
            ),
            (
                AuthError::NotAllowed,
                "AUTH_NOT_ALLOWED",
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
    fn test_message_scenarios() {
        let cases = vec![
            (
                AuthError::InvalidCredentials,
                "Invalid email or password",
            ),
            (
                AuthError::NotAllowed,
                "User not authorized to access \
                 this platform",
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

    #[test]
    fn test_status_scenarios() {
        let cases = vec![
            (
                AuthError::InvalidCredentials,
                StatusCode::UNAUTHORIZED,
            ),
            (
                AuthError::NotAllowed,
                StatusCode::FORBIDDEN,
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

    #[test]
    fn test_display_scenarios() {
        let cases = vec![
            (
                AuthError::InvalidCredentials,
                "Invalid credentials",
            ),
            (
                AuthError::NotAllowed,
                "User not authorized to access \
                 this platform",
            ),
        ];
        for (err, expected) in cases {
            assert_eq!(
                err.to_string(),
                expected,
                "display for {:?}",
                err
            );
        }
    }

    #[test]
    fn test_from_supabase_auth_maps_correctly() {
        let unauth = SupabaseAuthError::Http {
            status: StatusCode::UNAUTHORIZED,
            body: "bad".to_string(),
        };
        assert!(matches!(
            AuthError::from(unauth),
            AuthError::InvalidCredentials
        ));

        let bad_req = SupabaseAuthError::Http {
            status: StatusCode::BAD_REQUEST,
            body: "err".to_string(),
        };
        assert!(matches!(
            AuthError::from(bad_req),
            AuthError::InvalidCredentials
        ));

        let server = SupabaseAuthError::Http {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: "500".to_string(),
        };
        assert!(matches!(
            AuthError::from(server),
            AuthError::External(_)
        ));

        let parse = SupabaseAuthError::Parse {
            body: "bad json".to_string(),
        };
        assert!(matches!(
            AuthError::from(parse),
            AuthError::External(_)
        ));
    }

    #[test]
    fn test_source_credentials_is_none() {
        use std::error::Error;
        let err = AuthError::InvalidCredentials;
        assert!(err.source().is_none());
    }

    #[test]
    fn test_source_not_allowed_is_none() {
        use std::error::Error;
        let err = AuthError::NotAllowed;
        assert!(err.source().is_none());
    }
}
