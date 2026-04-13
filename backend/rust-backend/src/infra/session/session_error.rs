//! Session error - Session-related errors
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;

/// Session-related errors
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    /// Session not found
    #[error("Session not found")]
    NotFound,
    /// Session expired
    #[allow(dead_code)]
    #[error("Session expired")]
    Expired,
}

impl SessionError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound => "SESSION_NOT_FOUND",
            Self::Expired => "SESSION_EXPIRED",
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::NotFound => {
                "Session not found or expired. \
                 Please log in again."
            }
            Self::Expired => {
                "Session has expired. \
                 Please log in again."
            }
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::NotFound | Self::Expired => {
                StatusCode::UNAUTHORIZED
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_scenarios() {
        let cases = vec![
            (SessionError::NotFound, "SESSION_NOT_FOUND"),
            (SessionError::Expired, "SESSION_EXPIRED"),
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
                SessionError::NotFound,
                "Session not found or expired. \
                 Please log in again.",
            ),
            (
                SessionError::Expired,
                "Session has expired. \
                 Please log in again.",
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
    fn test_status_both_return_unauthorized() {
        let cases = vec![
            SessionError::NotFound,
            SessionError::Expired,
        ];
        for err in cases {
            assert_eq!(
                err.status(),
                StatusCode::UNAUTHORIZED,
                "status() for {:?}",
                err
            );
        }
    }

    #[test]
    fn test_display_scenarios() {
        let cases = vec![
            (SessionError::NotFound, "Session not found"),
            (SessionError::Expired, "Session expired"),
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
}
