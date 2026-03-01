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
            Self::AlreadyExists(name) => {
                write!(f, "App '{}' already exists", name)
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    // -- code() tests --

    #[test]
    fn test_code_scenarios() {
        let cases = vec![
            (
                AppError::NotFound("x".into()),
                "APP_NOT_FOUND",
            ),
            (
                AppError::AlreadyExists("x".into()),
                "APP_ALREADY_EXISTS",
            ),
            (
                AppError::UserAppAlreadyAdded {
                    user_id: "u".into(),
                    app_name: "a".into(),
                },
                "USER_APP_ALREADY_ADDED",
            ),
            (
                AppError::UserAppNotFound {
                    user_id: "u".into(),
                    app_name: "a".into(),
                },
                "USER_APP_NOT_FOUND",
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

    // -- message() tests --

    #[test]
    fn test_message_scenarios() {
        let cases = vec![
            (
                AppError::NotFound("x".into()),
                "App not found",
            ),
            (
                AppError::AlreadyExists("x".into()),
                "App already exists",
            ),
            (
                AppError::UserAppAlreadyAdded {
                    user_id: "u".into(),
                    app_name: "a".into(),
                },
                "User already has this app",
            ),
            (
                AppError::UserAppNotFound {
                    user_id: "u".into(),
                    app_name: "a".into(),
                },
                "User doesn't have this app",
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

    // -- status() tests --

    #[test]
    fn test_status_scenarios() {
        let cases = vec![
            (
                AppError::NotFound("x".into()),
                StatusCode::NOT_FOUND,
            ),
            (
                AppError::AlreadyExists("x".into()),
                StatusCode::CONFLICT,
            ),
            (
                AppError::UserAppAlreadyAdded {
                    user_id: "u".into(),
                    app_name: "a".into(),
                },
                StatusCode::CONFLICT,
            ),
            (
                AppError::UserAppNotFound {
                    user_id: "u".into(),
                    app_name: "a".into(),
                },
                StatusCode::NOT_FOUND,
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

    // -- Display tests --

    #[test]
    fn test_display_not_found() {
        // arrange
        let err = AppError::NotFound("intello".into());

        // act / assert
        assert_eq!(
            err.to_string(),
            "App 'intello' not found"
        );
    }

    #[test]
    fn test_display_already_exists() {
        // arrange
        let err = AppError::AlreadyExists("intello".into());

        // act / assert
        assert_eq!(
            err.to_string(),
            "App 'intello' already exists"
        );
    }

    #[test]
    fn test_display_user_app_already_added() {
        // arrange
        let err = AppError::UserAppAlreadyAdded {
            user_id: "u-1".into(),
            app_name: "intello".into(),
        };

        // act / assert
        assert_eq!(
            err.to_string(),
            "User 'u-1' already has app 'intello'"
        );
    }

    #[test]
    fn test_display_user_app_not_found() {
        // arrange
        let err = AppError::UserAppNotFound {
            user_id: "u-2".into(),
            app_name: "quiz".into(),
        };

        // act / assert
        assert_eq!(
            err.to_string(),
            "User 'u-2' doesn't have app 'quiz'"
        );
    }
}
