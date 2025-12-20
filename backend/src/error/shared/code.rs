//! Error codes - maps enum variants to constants

use crate::shared::constants::errors::{codes, messages, status};
use actix_web::http::StatusCode;

// == ERROR CODE ENUM // ==

/// Centralized error codes for consistent API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Auth
    InvalidCredentials,
    // Supabase
    SupabaseHttpError,
    SupabaseNetworkError,
    SupabaseParseError,
    SupabaseTimeout,
    // Validation
    ValidationFailed,
    // Collection
    CollectionDvdNotFound,
    CollectionDvdDuplicate,
    CollectionUserNotFound,
    CollectionStorageError,
    // Session
    SessionNotFound,
    SessionExpired,
    // Intello
    IntelloQcmSetNotFound,
    IntelloGameSetNotFound,  // Generic: works for any game type
    IntelloValidationFailed,
    IntelloStorageError,
    // Apps
    AppNotFound,
    AppAlreadyExists,
    UserAppAlreadyAdded,
    UserAppNotFound,
    // Internal
    InternalError,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => codes::AUTH_INVALID_CREDENTIALS,
            Self::SupabaseHttpError => codes::SUPABASE_HTTP_ERROR,
            Self::SupabaseNetworkError => codes::SUPABASE_NETWORK_ERROR,
            Self::SupabaseParseError => codes::SUPABASE_PARSE_ERROR,
            Self::SupabaseTimeout => codes::SUPABASE_TIMEOUT,
            Self::ValidationFailed => codes::VALIDATION_FAILED,
            // Collection
            Self::CollectionDvdNotFound => codes::COLLECTION_DVD_NOT_FOUND,
            Self::CollectionDvdDuplicate => codes::COLLECTION_DVD_DUPLICATE,
            Self::CollectionUserNotFound => codes::COLLECTION_USER_NOT_FOUND,
            Self::CollectionStorageError => codes::COLLECTION_STORAGE_ERROR,
            // Session
            Self::SessionNotFound => codes::SESSION_NOT_FOUND,
            Self::SessionExpired => codes::SESSION_EXPIRED,
            // Intello
            Self::IntelloQcmSetNotFound => codes::INTELLO_QCM_SET_NOT_FOUND,
            Self::IntelloGameSetNotFound => "INTELLO_GAME_SET_NOT_FOUND",
            Self::IntelloValidationFailed => codes::INTELLO_VALIDATION_FAILED,
            Self::IntelloStorageError => codes::INTELLO_STORAGE_ERROR,
            // Apps
            Self::AppNotFound => "APP_NOT_FOUND",
            Self::AppAlreadyExists => "APP_ALREADY_EXISTS",
            Self::UserAppAlreadyAdded => "USER_APP_ALREADY_ADDED",
            Self::UserAppNotFound => "USER_APP_NOT_FOUND",
            // Internal
            Self::InternalError => "INTERNAL_ERROR",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => messages::AUTH_INVALID_CREDENTIALS,
            Self::SupabaseHttpError => messages::SUPABASE_HTTP_ERROR,
            Self::SupabaseNetworkError => messages::SUPABASE_NETWORK_ERROR,
            Self::SupabaseParseError => messages::SUPABASE_PARSE_ERROR,
            Self::SupabaseTimeout => messages::SUPABASE_TIMEOUT,
            Self::ValidationFailed => messages::VALIDATION_FAILED,
            // Collection
            Self::CollectionDvdNotFound => messages::COLLECTION_DVD_NOT_FOUND,
            Self::CollectionDvdDuplicate => messages::COLLECTION_DVD_DUPLICATE,
            Self::CollectionUserNotFound => messages::COLLECTION_USER_NOT_FOUND,
            Self::CollectionStorageError => messages::COLLECTION_STORAGE_ERROR,
            // Session
            Self::SessionNotFound => messages::SESSION_NOT_FOUND,
            Self::SessionExpired => messages::SESSION_EXPIRED,
            // Intello
            Self::IntelloQcmSetNotFound => messages::INTELLO_QCM_SET_NOT_FOUND,
            Self::IntelloGameSetNotFound => "Game set not found",
            Self::IntelloValidationFailed => messages::INTELLO_VALIDATION_FAILED,
            Self::IntelloStorageError => messages::INTELLO_STORAGE_ERROR,
            // Apps
            Self::AppNotFound => "App not found",
            Self::AppAlreadyExists => "App already exists",
            Self::UserAppAlreadyAdded => "User already has this app",
            Self::UserAppNotFound => "User doesn't have this app",
            // Internal
            Self::InternalError => "Internal server error",
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => status::AUTH_INVALID_CREDENTIALS,
            Self::SupabaseHttpError => status::SUPABASE_HTTP_ERROR,
            Self::SupabaseNetworkError => status::SUPABASE_NETWORK_ERROR,
            Self::SupabaseParseError => status::SUPABASE_PARSE_ERROR,
            Self::SupabaseTimeout => status::SUPABASE_TIMEOUT,
            Self::ValidationFailed => status::VALIDATION_FAILED,
            // Collection
            Self::CollectionDvdNotFound => status::COLLECTION_DVD_NOT_FOUND,
            Self::CollectionDvdDuplicate => status::COLLECTION_DVD_DUPLICATE,
            Self::CollectionUserNotFound => status::COLLECTION_USER_NOT_FOUND,
            Self::CollectionStorageError => status::COLLECTION_STORAGE_ERROR,
            // Session
            Self::SessionNotFound => status::SESSION_NOT_FOUND,
            Self::SessionExpired => status::SESSION_EXPIRED,
            // Intello
            Self::IntelloQcmSetNotFound => status::INTELLO_QCM_SET_NOT_FOUND,
            Self::IntelloGameSetNotFound => StatusCode::NOT_FOUND,
            Self::IntelloValidationFailed => status::INTELLO_VALIDATION_FAILED,
            Self::IntelloStorageError => status::INTELLO_STORAGE_ERROR,
            // Apps
            Self::AppNotFound => StatusCode::NOT_FOUND,
            Self::AppAlreadyExists => StatusCode::CONFLICT,
            Self::UserAppAlreadyAdded => StatusCode::CONFLICT,
            Self::UserAppNotFound => StatusCode::NOT_FOUND,
            // Internal
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
