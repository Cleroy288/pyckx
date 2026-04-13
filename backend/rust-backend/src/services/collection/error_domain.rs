//! Collection error - Service layer errors
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;

/// Errors during collection operations
#[derive(Debug, thiserror::Error)]
pub enum CollectionError {
    /// DVD not found in collection
    #[error("DVD not found: {dvd_id}")]
    DvdNotFound { dvd_id: String },
    /// DVD with same name already exists
    #[error("DVD already exists: {name}")]
    DvdDuplicate { name: String },
    /// User collection not found
    #[error("User collection not found: {user_id}")]
    UserNotFound { user_id: String },
    /// Duplicate collection (unique constraint violation)
    #[error("Collection already exists for user {user_id}")]
    Duplicate { user_id: String },
    /// Storage operation failed
    #[error("Storage error: {message}")]
    StorageError { message: String },
}

impl CollectionError {
    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::DvdNotFound { .. } => {
                "COLLECTION_DVD_NOT_FOUND"
            }
            Self::DvdDuplicate { .. } => {
                "COLLECTION_DVD_DUPLICATE"
            }
            Self::UserNotFound { .. } => {
                "COLLECTION_USER_NOT_FOUND"
            }
            Self::Duplicate { .. } => {
                "COLLECTION_DUPLICATE"
            }
            Self::StorageError { .. } => {
                "COLLECTION_STORAGE_ERROR"
            }
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::DvdNotFound { .. } => {
                "DVD not found in collection"
            }
            Self::DvdDuplicate { .. } => {
                "A DVD with this name already exists \
                 in your collection"
            }
            Self::UserNotFound { .. } => {
                "User collection not found"
            }
            Self::Duplicate { .. } => {
                "This collection already exists"
            }
            Self::StorageError { .. } => {
                "Failed to access collection storage"
            }
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::DvdNotFound { .. }
            | Self::UserNotFound { .. } => {
                StatusCode::NOT_FOUND
            }
            Self::DvdDuplicate { .. }
            | Self::Duplicate { .. } => {
                StatusCode::CONFLICT
            }
            Self::StorageError { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// Create a DvdNotFound error
    pub fn dvd_not_found(
        dvd_id: impl Into<String>,
    ) -> Self {
        Self::DvdNotFound {
            dvd_id: dvd_id.into(),
        }
    }

    /// Create a DvdDuplicate error
    pub fn dvd_duplicate(
        name: impl Into<String>,
    ) -> Self {
        Self::DvdDuplicate { name: name.into() }
    }
}

impl CollectionError {
    /// Create a UserNotFound error
    #[allow(dead_code)]
    pub fn user_not_found(
        user_id: impl Into<String>,
    ) -> Self {
        Self::UserNotFound {
            user_id: user_id.into(),
        }
    }

    /// Create a Duplicate error
    pub fn duplicate(
        user_id: impl Into<String>,
    ) -> Self {
        Self::Duplicate {
            user_id: user_id.into(),
        }
    }

    /// Create a StorageError
    pub fn storage_error(
        message: impl Into<String>,
    ) -> Self {
        Self::StorageError {
            message: message.into(),
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

    #[test]
    fn test_message_scenarios() {
        let cases = vec![
            (
                CollectionError::dvd_not_found("d1"),
                "DVD not found in collection",
            ),
            (
                CollectionError::dvd_duplicate("Matrix"),
                "A DVD with this name already exists \
                 in your collection",
            ),
            (
                CollectionError::storage_error("timeout"),
                "Failed to access collection storage",
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

    #[test]
    fn test_display_scenarios() {
        let cases = vec![
            (
                CollectionError::dvd_not_found("dvd-99"),
                "DVD not found: dvd-99",
            ),
            (
                CollectionError::dvd_duplicate("Alien"),
                "DVD already exists: Alien",
            ),
            (
                CollectionError::storage_error("db down"),
                "Storage error: db down",
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
    fn test_constructor_stores_fields() {
        let dvd = CollectionError::dvd_not_found("dvd-42");
        match dvd {
            CollectionError::DvdNotFound { dvd_id } => {
                assert_eq!(dvd_id, "dvd-42");
            }
            _ => panic!("Expected DvdNotFound"),
        }

        let dup = CollectionError::dvd_duplicate("Matrix");
        match dup {
            CollectionError::DvdDuplicate { name } => {
                assert_eq!(name, "Matrix");
            }
            _ => panic!("Expected DvdDuplicate"),
        }
    }
}
