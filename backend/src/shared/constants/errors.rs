//! Error constants - codes, messages, and HTTP status mappings.
//!
//! All error-related constants live here. The error module
//! imports these for consistent error responses across the application.

use actix_web::http::StatusCode;

// ============================================================================
// ERROR CODES - String identifiers sent to clients
// ============================================================================

pub mod codes {
    // Auth
    pub const AUTH_INVALID_CREDENTIALS: &str = "AUTH_INVALID_CREDENTIALS";

    // Supabase
    pub const SUPABASE_HTTP_ERROR: &str = "SUPABASE_HTTP_ERROR";
    pub const SUPABASE_NETWORK_ERROR: &str = "SUPABASE_NETWORK_ERROR";
    pub const SUPABASE_PARSE_ERROR: &str = "SUPABASE_PARSE_ERROR";
    pub const SUPABASE_TIMEOUT: &str = "SUPABASE_TIMEOUT";

    // Validation
    pub const VALIDATION_FAILED: &str = "VALIDATION_FAILED";

    // == COLLECTION ERRORS // ==
    pub const COLLECTION_DVD_NOT_FOUND: &str = "COLLECTION_DVD_NOT_FOUND";
    pub const COLLECTION_DVD_DUPLICATE: &str = "COLLECTION_DVD_DUPLICATE";
    pub const COLLECTION_USER_NOT_FOUND: &str = "COLLECTION_USER_NOT_FOUND";
    pub const COLLECTION_STORAGE_ERROR: &str = "COLLECTION_STORAGE_ERROR";

    // == SESSION ERRORS // ==
    pub const SESSION_NOT_FOUND: &str = "SESSION_NOT_FOUND";
    pub const SESSION_EXPIRED: &str = "SESSION_EXPIRED";

    // == INTELLO ERRORS // ==
    pub const INTELLO_QCM_SET_NOT_FOUND: &str = "INTELLO_QCM_SET_NOT_FOUND";
    pub const INTELLO_VALIDATION_FAILED: &str = "INTELLO_VALIDATION_FAILED";
    pub const INTELLO_STORAGE_ERROR: &str = "INTELLO_STORAGE_ERROR";
}

// ============================================================================
// ERROR MESSAGES - User-friendly messages (safe to expose to clients)
// ============================================================================

pub mod messages {
    // Auth
    pub const AUTH_INVALID_CREDENTIALS: &str = "Invalid email or password";

    // Supabase
    pub const SUPABASE_HTTP_ERROR: &str = "Authentication service error";
    pub const SUPABASE_NETWORK_ERROR: &str = "Unable to reach authentication service";
    pub const SUPABASE_PARSE_ERROR: &str = "Authentication service returned invalid data";
    pub const SUPABASE_TIMEOUT: &str = "Authentication service timed out";

    // Validation
    pub const VALIDATION_FAILED: &str = "Invalid input data";

    // == COLLECTION ERRORS // ==
    pub const COLLECTION_DVD_NOT_FOUND: &str = "DVD not found in collection";
    pub const COLLECTION_DVD_DUPLICATE: &str = "A DVD with this name already exists in your collection";
    pub const COLLECTION_USER_NOT_FOUND: &str = "User collection not found";
    pub const COLLECTION_STORAGE_ERROR: &str = "Failed to access collection storage";

    // == SESSION ERRORS // ==
    pub const SESSION_NOT_FOUND: &str = "Session not found or expired. Please log in again.";
    pub const SESSION_EXPIRED: &str = "Session has expired. Please log in again.";

    // == INTELLO ERRORS // ==
    pub const INTELLO_QCM_SET_NOT_FOUND: &str = "QCM set not found";
    pub const INTELLO_VALIDATION_FAILED: &str = "Invalid QCM set data";
    pub const INTELLO_STORAGE_ERROR: &str = "Failed to access Intello storage";
}

// ============================================================================
// HTTP STATUS MAPPING
// ============================================================================

pub mod status {
    use super::*;

    pub const AUTH_INVALID_CREDENTIALS: StatusCode = StatusCode::UNAUTHORIZED;

    pub const SUPABASE_HTTP_ERROR: StatusCode = StatusCode::BAD_GATEWAY;
    pub const SUPABASE_NETWORK_ERROR: StatusCode = StatusCode::BAD_GATEWAY;
    pub const SUPABASE_PARSE_ERROR: StatusCode = StatusCode::BAD_GATEWAY;
    pub const SUPABASE_TIMEOUT: StatusCode = StatusCode::GATEWAY_TIMEOUT;

    pub const VALIDATION_FAILED: StatusCode = StatusCode::BAD_REQUEST;

    // == COLLECTION ERRORS // ==
    pub const COLLECTION_DVD_NOT_FOUND: StatusCode = StatusCode::NOT_FOUND;
    pub const COLLECTION_DVD_DUPLICATE: StatusCode = StatusCode::CONFLICT;
    pub const COLLECTION_USER_NOT_FOUND: StatusCode = StatusCode::NOT_FOUND;
    pub const COLLECTION_STORAGE_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;

    // == SESSION ERRORS // ==
    pub const SESSION_NOT_FOUND: StatusCode = StatusCode::UNAUTHORIZED;
    pub const SESSION_EXPIRED: StatusCode = StatusCode::UNAUTHORIZED;

    // == INTELLO ERRORS // ==
    pub const INTELLO_QCM_SET_NOT_FOUND: StatusCode = StatusCode::NOT_FOUND;
    pub const INTELLO_VALIDATION_FAILED: StatusCode = StatusCode::BAD_REQUEST;
    pub const INTELLO_STORAGE_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
}
