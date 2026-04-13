//! Supabase auth error - Infrastructure layer errors
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
use serde::de::DeserializeOwned;

/// Max body length logged in Display impl
const MAX_BODY_LEN: usize = 200;

/// Errors from Supabase auth API calls
#[derive(Debug, thiserror::Error)]
pub enum SupabaseAuthError {
    /// HTTP error response (4xx, 5xx)
    #[error(
        "Supabase HTTP {status} - {}",
        truncate_body(body)
    )]
    Http { status: StatusCode, body: String },
    /// Network/connection error
    #[error("Supabase network error: {0}")]
    Network(reqwest::Error),
    /// JSON parsing error
    #[error(
        "Supabase parse error, body: {}",
        truncate_body(body)
    )]
    Parse { body: String },
    /// Request timeout
    #[error("Supabase timeout: {0}")]
    Timeout(reqwest::Error),
}

/// Truncate body to MAX_BODY_LEN chars for safe logging
fn truncate_body(body: &str) -> String {
    if body.len() <= MAX_BODY_LEN {
        return body.to_string();
    }
    let truncated = &body[..MAX_BODY_LEN];
    format!("{}...[truncated]", truncated)
}

impl SupabaseAuthError {
    /// Create from reqwest error
    pub fn from_reqwest(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            return Self::Timeout(err);
        }
        if err.is_connect() || err.is_request() {
            return Self::Network(err);
        }
        if err.is_decode() {
            return Self::Parse {
                body: String::new(),
            };
        }
        if let Some(s) = err.status() {
            return Self::Http {
                status: StatusCode::from_u16(s.as_u16())
                    .unwrap_or(StatusCode::BAD_GATEWAY),
                body: String::new(),
            };
        }
        Self::Network(err)
    }

    /// Create HTTP error from status and body
    pub fn http(
        status: reqwest::StatusCode,
        body: String,
    ) -> Self {
        Self::Http {
            status: StatusCode::from_u16(status.as_u16())
                .unwrap_or(StatusCode::BAD_GATEWAY),
            body,
        }
    }

    /// Create parse error from body
    pub fn parse(body: String) -> Self {
        Self::Parse { body }
    }

    /// Error code string for API responses
    pub fn code(&self) -> &'static str {
        match self {
            Self::Http { status, .. }
                if *status == StatusCode::UNAUTHORIZED
                    || *status
                        == StatusCode::BAD_REQUEST =>
            {
                "AUTH_INVALID_CREDENTIALS"
            }
            Self::Http { .. } => "SUPABASE_HTTP_ERROR",
            Self::Network(_) => "SUPABASE_NETWORK_ERROR",
            Self::Parse { .. } => "SUPABASE_PARSE_ERROR",
            Self::Timeout(_) => "SUPABASE_TIMEOUT",
        }
    }
}

impl SupabaseAuthError {
    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::Http { status, .. }
                if *status == StatusCode::UNAUTHORIZED
                    || *status
                        == StatusCode::BAD_REQUEST =>
            {
                "Invalid email or password"
            }
            Self::Http { .. } => {
                "Authentication service error"
            }
            Self::Network(_) => {
                "Unable to reach authentication service"
            }
            Self::Parse { .. } => {
                "Authentication service returned \
                 invalid data"
            }
            Self::Timeout(_) => {
                "Authentication service timed out"
            }
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Http { status, .. }
                if *status == StatusCode::UNAUTHORIZED
                    || *status
                        == StatusCode::BAD_REQUEST =>
            {
                StatusCode::UNAUTHORIZED
            }
            Self::Http { .. }
            | Self::Network(_)
            | Self::Parse { .. } => {
                StatusCode::BAD_GATEWAY
            }
            Self::Timeout(_) => {
                StatusCode::GATEWAY_TIMEOUT
            }
        }
    }

    /// Parse a reqwest Response into expected type T
    pub async fn parse_response<T: DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, Self> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await;
            let body = log_and_default_body(body);
            return Err(Self::http(status, body));
        }
        let body_text = response
            .text()
            .await
            .map_err(Self::from_reqwest)?;
        serde_json::from_str::<T>(&body_text)
            .map_err(|_| Self::parse(body_text))
    }
}

/// Log warning on body read failure, return fallback
fn log_and_default_body(
    result: Result<String, reqwest::Error>,
) -> String {
    match result {
        Ok(body) => body,
        Err(err) => {
            tracing::warn!(
                error = %err,
                "Failed to read Supabase error body"
            );
            String::from("<unreadable body>")
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
                SupabaseAuthError::Http {
                    status: StatusCode::UNAUTHORIZED,
                    body: "bad".into(),
                },
                "AUTH_INVALID_CREDENTIALS",
            ),
            (
                SupabaseAuthError::Http {
                    status: StatusCode::BAD_REQUEST,
                    body: "err".into(),
                },
                "AUTH_INVALID_CREDENTIALS",
            ),
            (
                SupabaseAuthError::Http {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    body: "500".into(),
                },
                "SUPABASE_HTTP_ERROR",
            ),
            (
                SupabaseAuthError::parse("bad json".into()),
                "SUPABASE_PARSE_ERROR",
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
                SupabaseAuthError::Http {
                    status: StatusCode::UNAUTHORIZED,
                    body: String::new(),
                },
                "Invalid email or password",
            ),
            (
                SupabaseAuthError::Http {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    body: String::new(),
                },
                "Authentication service error",
            ),
            (
                SupabaseAuthError::parse("x".into()),
                "Authentication service returned \
                 invalid data",
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
                SupabaseAuthError::Http {
                    status: StatusCode::UNAUTHORIZED,
                    body: String::new(),
                },
                StatusCode::UNAUTHORIZED,
            ),
            (
                SupabaseAuthError::Http {
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    body: String::new(),
                },
                StatusCode::BAD_GATEWAY,
            ),
            (
                SupabaseAuthError::parse("x".into()),
                StatusCode::BAD_GATEWAY,
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
    fn test_truncate_body_short_not_truncated() {
        let body = "short body";
        assert_eq!(truncate_body(body), "short body");
    }

    #[test]
    fn test_truncate_body_long_is_truncated() {
        let body = "x".repeat(300);
        let result = truncate_body(&body);
        assert!(result.contains("[truncated]"));
        assert!(result.len() < 300);
    }
}
