/*
 * Supabase HTTP Error Types
 *
 * Provides typed errors for all Supabase HTTP operations with retry support.
 * Used by SupabaseHttpClient for consistent error handling across repositories.
 */

use std::fmt;

/* ============================================================================
 * SUPABASE ERROR ENUM
 * ============================================================================ */

/*
 * Error types for Supabase HTTP operations.
 *
 * Variants:
 * - Http: Non-success HTTP status returned (4xx, 5xx)
 * - Network: Connection or network failure
 * - Parse: Response body could not be parsed as JSON
 * - Timeout: Request exceeded configured timeout
 */
#[derive(Debug, Clone)]
pub enum SupabaseError {
    /* HTTP error with status code and response body */
    Http { status: u16, body: String },

    /* Network connectivity error */
    Network(String),

    /* JSON parsing error */
    Parse(String),

    /* Request timeout */
    Timeout,
}

/* ============================================================================
 * ERROR IMPLEMENTATION
 * ============================================================================ */

impl SupabaseError {
    /*
     * Check if error is retryable.
     *
     * Returns true for:
     * - 5xx server errors
     * - 429 rate limiting
     * - Network errors
     * - Timeouts
     */
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http { status, .. } => *status >= 500 || *status == 429,
            Self::Network(_) | Self::Timeout => true,
            Self::Parse(_) => false,
        }
    }

    /* Create HTTP error from status and body */
    pub fn http(status: u16, body: impl Into<String>) -> Self {
        Self::Http { status, body: body.into() }
    }

    /* Create network error */
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /* Create parse error */
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }
}

/* ============================================================================
 * DISPLAY IMPLEMENTATION
 * ============================================================================ */

impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http { status, body } => write!(f, "HTTP {}: {}", status, body),
            Self::Network(msg) => write!(f, "Network error: {}", msg),
            Self::Parse(msg) => write!(f, "Parse error: {}", msg),
            Self::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl std::error::Error for SupabaseError {}
