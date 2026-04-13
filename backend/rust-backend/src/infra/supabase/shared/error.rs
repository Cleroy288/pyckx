//! Supabase HTTP error types
//!
//! Typed errors for all Supabase HTTP operations
//! with retry support. Used by SupabaseHttpClient.

/// Error types for Supabase HTTP operations
#[derive(Debug, Clone, thiserror::Error)]
pub enum SupabaseError {
    /// HTTP error with status code and response body
    #[error("HTTP {status}: {body}")]
    Http { status: u16, body: String },
    /// Network connectivity error
    #[error("Network error: {0}")]
    Network(String),
    /// JSON parsing error
    #[error("Parse error: {0}")]
    Parse(String),
    /// Request timeout
    #[error("Request timeout")]
    Timeout,
}

impl SupabaseError {
    /// Check if error is retryable (5xx, 429, network, timeout)
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Http { status, .. } => {
                *status >= 500 || *status == 429
            }
            Self::Network(_) | Self::Timeout => true,
            Self::Parse(_) => false,
        }
    }

    /// Create HTTP error from status and body
    pub fn http(
        status: u16,
        body: impl Into<String>,
    ) -> Self {
        Self::Http {
            status,
            body: body.into(),
        }
    }

    /// Create network error
    pub fn network(msg: impl Into<String>) -> Self {
        Self::Network(msg.into())
    }

    /// Create parse error
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_retryable_scenarios() {
        let cases = vec![
            (SupabaseError::http(500, "err"), true),
            (SupabaseError::http(429, "limit"), true),
            (SupabaseError::http(503, "unavail"), true),
            (SupabaseError::http(400, "bad"), false),
            (SupabaseError::http(404, "nope"), false),
            (SupabaseError::network("reset"), true),
            (SupabaseError::Timeout, true),
            (SupabaseError::parse("bad json"), false),
        ];
        for (err, expected) in cases {
            assert_eq!(
                err.is_retryable(),
                expected,
                "is_retryable for {:?}",
                err
            );
        }
    }

    #[test]
    fn test_constructor_stores_fields() {
        match SupabaseError::http(422, "unprocessable") {
            SupabaseError::Http { status, body } => {
                assert_eq!(status, 422);
                assert_eq!(body, "unprocessable");
            }
            _ => panic!("Expected Http"),
        }
        match SupabaseError::network("dns failure") {
            SupabaseError::Network(msg) => {
                assert_eq!(msg, "dns failure");
            }
            _ => panic!("Expected Network"),
        }
        match SupabaseError::parse("unexpected token") {
            SupabaseError::Parse(msg) => {
                assert_eq!(msg, "unexpected token");
            }
            _ => panic!("Expected Parse"),
        }
    }

    #[test]
    fn test_display_scenarios() {
        let cases = vec![
            (
                SupabaseError::http(500, "internal"),
                "HTTP 500: internal",
            ),
            (
                SupabaseError::network("timeout"),
                "Network error: timeout",
            ),
            (
                SupabaseError::parse("bad json"),
                "Parse error: bad json",
            ),
            (
                SupabaseError::Timeout,
                "Request timeout",
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
