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
        Self::Http {
            status,
            body: body.into(),
        }
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
            Self::Http { status, body } => {
                write!(f, "HTTP {}: {}", status, body)
            }
            Self::Network(msg) => write!(f, "Network error: {}", msg),
            Self::Parse(msg) => write!(f, "Parse error: {}", msg),
            Self::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl std::error::Error for SupabaseError {}

#[cfg(test)]
mod tests {
    use super::*;

    // -- is_retryable() tests --

    #[test]
    fn test_is_retryable_500_returns_true() {
        // arrange
        let err = SupabaseError::http(500, "server error");

        // act / assert
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_429_returns_true() {
        // arrange
        let err = SupabaseError::http(429, "rate limited");

        // act / assert
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_503_returns_true() {
        // arrange
        let err = SupabaseError::http(503, "unavailable");

        // act / assert
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_400_returns_false() {
        // arrange
        let err = SupabaseError::http(400, "bad request");

        // act / assert
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_retryable_404_returns_false() {
        // arrange
        let err = SupabaseError::http(404, "not found");

        // act / assert
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_is_retryable_network_returns_true() {
        // arrange
        let err = SupabaseError::network("connection reset");

        // act / assert
        assert!(err.is_retryable());
    }

    #[test]
    fn test_is_retryable_timeout_returns_true() {
        // arrange / act / assert
        assert!(SupabaseError::Timeout.is_retryable());
    }

    #[test]
    fn test_is_retryable_parse_returns_false() {
        // arrange
        let err = SupabaseError::parse("invalid json");

        // act / assert
        assert!(!err.is_retryable());
    }

    // -- constructor tests --

    #[test]
    fn test_http_constructor_stores_fields() {
        // arrange / act
        let err = SupabaseError::http(422, "unprocessable");

        // assert
        match err {
            SupabaseError::Http { status, body } => {
                assert_eq!(status, 422);
                assert_eq!(body, "unprocessable");
            }
            _ => panic!("Expected Http variant"),
        }
    }

    #[test]
    fn test_network_constructor_stores_message() {
        // arrange / act
        let err = SupabaseError::network("dns failure");

        // assert
        match err {
            SupabaseError::Network(msg) => {
                assert_eq!(msg, "dns failure");
            }
            _ => panic!("Expected Network variant"),
        }
    }

    #[test]
    fn test_parse_constructor_stores_message() {
        // arrange / act
        let err = SupabaseError::parse("unexpected token");

        // assert
        match err {
            SupabaseError::Parse(msg) => {
                assert_eq!(msg, "unexpected token");
            }
            _ => panic!("Expected Parse variant"),
        }
    }

    // -- Display tests --

    #[test]
    fn test_display_http() {
        // arrange
        let err = SupabaseError::http(500, "internal");

        // act / assert
        assert_eq!(err.to_string(), "HTTP 500: internal");
    }

    #[test]
    fn test_display_network() {
        // arrange
        let err = SupabaseError::network("timeout");

        // act / assert
        assert_eq!(err.to_string(), "Network error: timeout");
    }

    #[test]
    fn test_display_parse() {
        // arrange
        let err = SupabaseError::parse("bad json");

        // act / assert
        assert_eq!(err.to_string(), "Parse error: bad json");
    }

    #[test]
    fn test_display_timeout() {
        // arrange / act / assert
        assert_eq!(
            SupabaseError::Timeout.to_string(),
            "Request timeout"
        );
    }
}
