//! Session error - Session-related errors
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
use std::fmt;

/// Session-related errors
#[derive(Debug)]
pub enum SessionError {
    /// Session not found
    NotFound,
    /// Session expired
    #[allow(dead_code)]
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
                "Session not found or expired. Please log in again."
            }
            Self::Expired => "Session has expired. Please log in again.",
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::UNAUTHORIZED,
            Self::Expired => StatusCode::UNAUTHORIZED,
        }
    }
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Session not found"),
            Self::Expired => write!(f, "Session expired"),
        }
    }
}

impl std::error::Error for SessionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_not_found() {
        // arrange / act / assert
        assert_eq!(SessionError::NotFound.code(), "SESSION_NOT_FOUND");
    }

    #[test]
    fn test_code_expired() {
        // arrange / act / assert
        assert_eq!(SessionError::Expired.code(), "SESSION_EXPIRED");
    }

    #[test]
    fn test_message_not_found() {
        // arrange / act
        let msg = SessionError::NotFound.message();

        // assert
        assert_eq!(
            msg,
            "Session not found or expired. Please log in again."
        );
    }

    #[test]
    fn test_message_expired() {
        // arrange / act
        let msg = SessionError::Expired.message();

        // assert
        assert_eq!(
            msg,
            "Session has expired. Please log in again."
        );
    }

    #[test]
    fn test_status_not_found_returns_unauthorized() {
        // arrange / act / assert
        assert_eq!(
            SessionError::NotFound.status(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[test]
    fn test_status_expired_returns_unauthorized() {
        // arrange / act / assert
        assert_eq!(
            SessionError::Expired.status(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[test]
    fn test_display_not_found() {
        // arrange / act / assert
        assert_eq!(
            SessionError::NotFound.to_string(),
            "Session not found"
        );
    }

    #[test]
    fn test_display_expired() {
        // arrange / act / assert
        assert_eq!(
            SessionError::Expired.to_string(),
            "Session expired"
        );
    }
}
