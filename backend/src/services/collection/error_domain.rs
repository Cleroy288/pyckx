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
            Self::DvdDuplicate { .. } => "A DVD with this name already exists in your collection",
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
            Self::DvdNotFound { dvd_id } => write!(f, "DVD not found: {}", dvd_id),
            Self::DvdDuplicate { name } => write!(f, "DVD already exists: {}", name),
            Self::UserNotFound { user_id } => write!(f, "User collection not found: {}", user_id),
            Self::StorageError { message } => write!(f, "Storage error: {}", message),
        }
    }
}

impl std::error::Error for CollectionError {}
