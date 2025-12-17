//! Collection error - Service layer errors for collection operations
//!
//! These errors are thrown by the CollectionService and converted
//! to AppError for HTTP responses.

use crate::error::shared::ErrorCode;
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
    /// Get the error code for this error
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::DvdNotFound { .. } => ErrorCode::CollectionDvdNotFound,
            Self::DvdDuplicate { .. } => ErrorCode::CollectionDvdDuplicate,
            Self::UserNotFound { .. } => ErrorCode::CollectionUserNotFound,
            Self::StorageError { .. } => ErrorCode::CollectionStorageError,
        }
    }

    /// Create a DvdNotFound error
    pub fn dvd_not_found(dvd_id: impl Into<String>) -> Self {
        Self::DvdNotFound { dvd_id: dvd_id.into() }
    }

    /// Create a DvdDuplicate error
    pub fn dvd_duplicate(name: impl Into<String>) -> Self {
        Self::DvdDuplicate { name: name.into() }
    }

    /// Create a UserNotFound error
    #[allow(dead_code)]
    pub fn user_not_found(user_id: impl Into<String>) -> Self {
        Self::UserNotFound { user_id: user_id.into() }
    }

    /// Create a StorageError
    pub fn storage_error(message: impl Into<String>) -> Self {
        Self::StorageError { message: message.into() }
    }
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DvdNotFound { dvd_id } => write!(f, "DVD not found: {}", dvd_id),
            Self::DvdDuplicate { name } => write!(f, "DVD already exists: {}", name),
            Self::UserNotFound { user_id } => write!(f, "User collection not found: {}", user_id),
            Self::StorageError { message } => write!(f, "Storage error: {}", message),
        }
    }
}

impl std::error::Error for CollectionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dvd_not_found_error() {
        let err = CollectionError::dvd_not_found("dvd-123");
        assert_eq!(err.code(), ErrorCode::CollectionDvdNotFound);
        assert!(err.to_string().contains("dvd-123"));
    }

    #[test]
    fn test_dvd_duplicate_error() {
        let err = CollectionError::dvd_duplicate("Inception");
        assert_eq!(err.code(), ErrorCode::CollectionDvdDuplicate);
        assert!(err.to_string().contains("Inception"));
    }

    #[test]
    fn test_user_not_found_error() {
        let err = CollectionError::user_not_found("user-456");
        assert_eq!(err.code(), ErrorCode::CollectionUserNotFound);
        assert!(err.to_string().contains("user-456"));
    }

    #[test]
    fn test_storage_error() {
        let err = CollectionError::storage_error("Failed to write file");
        assert_eq!(err.code(), ErrorCode::CollectionStorageError);
        assert!(err.to_string().contains("Failed to write file"));
    }
}
