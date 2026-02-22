//! Collection error - Service layer errors for collection operations
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
use std::fmt;

/// Errors that can occur during collection operations
#[derive(Debug)]
pub enum CollectionError {
    /// DVD not found in collection
    DvdNotFound { dvd_id: String },
    /// DVD with same name already exists
    DvdDuplicate { name: String },
    /// User collection not found
    UserNotFound { user_id: String },
    /// Storage operation failed
    StorageError { message: String },
}

impl CollectionError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::DvdNotFound { .. } => "COLLECTION_DVD_NOT_FOUND",
            Self::DvdDuplicate { .. } => "COLLECTION_DVD_DUPLICATE",
            Self::UserNotFound { .. } => "COLLECTION_USER_NOT_FOUND",
            Self::StorageError { .. } => "COLLECTION_STORAGE_ERROR",
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::DvdNotFound { .. } => "DVD not found in collection",
            Self::DvdDuplicate { .. } => {
                "A DVD with this name already exists in your collection"
            }
            Self::UserNotFound { .. } => "User collection not found",
            Self::StorageError { .. } => "Failed to access collection storage",
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::DvdNotFound { .. } => StatusCode::NOT_FOUND,
            Self::DvdDuplicate { .. } => StatusCode::CONFLICT,
            Self::UserNotFound { .. } => StatusCode::NOT_FOUND,
            Self::StorageError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Create a DvdNotFound error
    pub fn dvd_not_found(dvd_id: impl Into<String>) -> Self {
        Self::DvdNotFound {
            dvd_id: dvd_id.into(),
        }
    }

    /// Create a DvdDuplicate error
    pub fn dvd_duplicate(name: impl Into<String>) -> Self {
        Self::DvdDuplicate { name: name.into() }
    }

    /// Create a UserNotFound error
    #[allow(dead_code)]
    pub fn user_not_found(user_id: impl Into<String>) -> Self {
        Self::UserNotFound {
            user_id: user_id.into(),
        }
    }

    /// Create a StorageError
    pub fn storage_error(message: impl Into<String>) -> Self {
        Self::StorageError {
            message: message.into(),
        }
    }
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DvdNotFound { dvd_id } => {
                write!(f, "DVD not found: {}", dvd_id)
            }
            Self::DvdDuplicate { name } => {
                write!(f, "DVD already exists: {}", name)
            }
            Self::UserNotFound { user_id } => {
                write!(f, "User collection not found: {}", user_id)
            }
            Self::StorageError { message } => {
                write!(f, "Storage error: {}", message)
            }
        }
    }
}

impl std::error::Error for CollectionError {}

#[cfg(test)]
mod tests {
    use super::*;

    // -- code() tests --

    #[test]
    fn test_code_scenarios() {
        let cases = vec![
            (
                CollectionError::dvd_not_found("d1"),
                "COLLECTION_DVD_NOT_FOUND",
            ),
            (
                CollectionError::dvd_duplicate("Movie"),
                "COLLECTION_DVD_DUPLICATE",
            ),
            (
                CollectionError::user_not_found("u1"),
                "COLLECTION_USER_NOT_FOUND",
            ),
            (
                CollectionError::storage_error("err"),
                "COLLECTION_STORAGE_ERROR",
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
    fn test_message_dvd_not_found() {
        // arrange
        let err = CollectionError::dvd_not_found("d1");

        // act / assert
        assert_eq!(
            err.message(),
            "DVD not found in collection"
        );
    }

    #[test]
    fn test_message_dvd_duplicate() {
        // arrange
        let err = CollectionError::dvd_duplicate("Matrix");

        // act / assert
        assert_eq!(
            err.message(),
            "A DVD with this name already exists in your collection"
        );
    }

    #[test]
    fn test_message_storage_error() {
        // arrange
        let err = CollectionError::storage_error("timeout");

        // act / assert
        assert_eq!(
            err.message(),
            "Failed to access collection storage"
        );
    }

    // -- status() tests --

    #[test]
    fn test_status_scenarios() {
        let cases = vec![
            (
                CollectionError::dvd_not_found("d1"),
                StatusCode::NOT_FOUND,
            ),
            (
                CollectionError::dvd_duplicate("m"),
                StatusCode::CONFLICT,
            ),
            (
                CollectionError::user_not_found("u1"),
                StatusCode::NOT_FOUND,
            ),
            (
                CollectionError::storage_error("e"),
                StatusCode::INTERNAL_SERVER_ERROR,
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

    // -- constructor tests --

    #[test]
    fn test_dvd_not_found_stores_id() {
        // arrange / act
        let err = CollectionError::dvd_not_found("dvd-42");

        // assert
        match err {
            CollectionError::DvdNotFound { dvd_id } => {
                assert_eq!(dvd_id, "dvd-42");
            }
            _ => panic!("Expected DvdNotFound"),
        }
    }

    #[test]
    fn test_dvd_duplicate_stores_name() {
        // arrange / act
        let err = CollectionError::dvd_duplicate("Matrix");

        // assert
        match err {
            CollectionError::DvdDuplicate { name } => {
                assert_eq!(name, "Matrix");
            }
            _ => panic!("Expected DvdDuplicate"),
        }
    }

    // -- Display tests --

    #[test]
    fn test_display_dvd_not_found() {
        // arrange
        let err = CollectionError::dvd_not_found("dvd-99");

        // act / assert
        assert_eq!(err.to_string(), "DVD not found: dvd-99");
    }

    #[test]
    fn test_display_dvd_duplicate() {
        // arrange
        let err = CollectionError::dvd_duplicate("Alien");

        // act / assert
        assert_eq!(
            err.to_string(),
            "DVD already exists: Alien"
        );
    }

    #[test]
    fn test_display_storage_error() {
        // arrange
        let err = CollectionError::storage_error("db down");

        // act / assert
        assert_eq!(err.to_string(), "Storage error: db down");
    }
}
