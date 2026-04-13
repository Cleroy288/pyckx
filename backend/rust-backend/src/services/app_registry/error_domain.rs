//! App-specific errors
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;

/// App-specific error type
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    /// App not found in registry or database
    #[error("App '{0}' not found")]
    NotFound(String),
    /// App already exists
    #[error("App '{0}' already exists")]
    AlreadyExists(String),
    /// User already has this app
    #[error(
        "User '{user_id}' already has app '{app_name}'"
    )]
    UserAppAlreadyAdded {
        user_id: String,
        app_name: String,
    },
    /// User doesn't have this app
    #[error(
        "User '{user_id}' doesn't have app '{app_name}'"
    )]
    UserAppNotFound {
        user_id: String,
        app_name: String,
    },
}

impl AppError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "APP_NOT_FOUND",
            Self::AlreadyExists(_) => {
                "APP_ALREADY_EXISTS"
            }
            Self::UserAppAlreadyAdded { .. } => {
                "USER_APP_ALREADY_ADDED"
            }
            Self::UserAppNotFound { .. } => {
                "USER_APP_NOT_FOUND"
            }
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::NotFound(_) => "App not found",
            Self::AlreadyExists(_) => {
                "App already exists"
            }
            Self::UserAppAlreadyAdded { .. } => {
                "User already has this app"
            }
            Self::UserAppNotFound { .. } => {
                "User doesn't have this app"
            }
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::NotFound(_)
            | Self::UserAppNotFound { .. } => {
                StatusCode::NOT_FOUND
            }
            Self::AlreadyExists(_)
            | Self::UserAppAlreadyAdded { .. } => {
                StatusCode::CONFLICT
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

    #[test]
    fn test_display_scenarios() {
        let cases = vec![
            (
                AppError::NotFound("study".into()),
                "App 'study' not found",
            ),
            (
                AppError::AlreadyExists("study".into()),
                "App 'study' already exists",
            ),
            (
                AppError::UserAppAlreadyAdded {
                    user_id: "u-1".into(),
                    app_name: "study".into(),
                },
                "User 'u-1' already has app 'study'",
            ),
            (
                AppError::UserAppNotFound {
                    user_id: "u-2".into(),
                    app_name: "quiz".into(),
                },
                "User 'u-2' doesn't have app 'quiz'",
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
}
