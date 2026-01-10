//! Supabase error - Infrastructure layer errors
//!
//! Self-contained error with code, message, and status.

use actix_web::http::StatusCode;
use serde::de::DeserializeOwned;
use std::fmt;

/// Errors from Supabase API calls
#[derive(Debug)]
pub enum SupabaseError {
    /// HTTP error response (4xx, 5xx)
    Http { status: StatusCode, body: String },
    /// Network/connection error
    Network(reqwest::Error),
    /// JSON parsing error
    Parse { body: String },
    /// Request timeout
    Timeout(reqwest::Error),
}

impl SupabaseError {
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
                status: StatusCode::from_u16(s.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
                body: String::new(),
            };
        }
        Self::Network(err)
    }

    /// Create HTTP error from status and body
    pub fn http(status: reqwest::StatusCode, body: String) -> Self {
        Self::Http {
            status: StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
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
                if *status == StatusCode::UNAUTHORIZED || *status == StatusCode::BAD_REQUEST =>
            {
                "AUTH_INVALID_CREDENTIALS"
            }
            Self::Http { .. } => "SUPABASE_HTTP_ERROR",
            Self::Network(_) => "SUPABASE_NETWORK_ERROR",
            Self::Parse { .. } => "SUPABASE_PARSE_ERROR",
            Self::Timeout(_) => "SUPABASE_TIMEOUT",
        }
    }

    /// User-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            Self::Http { status, .. }
                if *status == StatusCode::UNAUTHORIZED || *status == StatusCode::BAD_REQUEST =>
            {
                "Invalid email or password"
            }
            Self::Http { .. } => "Authentication service error",
            Self::Network(_) => "Unable to reach authentication service",
            Self::Parse { .. } => "Authentication service returned invalid data",
            Self::Timeout(_) => "Authentication service timed out",
        }
    }

    /// HTTP status code
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Http { status, .. }
                if *status == StatusCode::UNAUTHORIZED || *status == StatusCode::BAD_REQUEST =>
            {
                StatusCode::UNAUTHORIZED
            }
            Self::Http { .. } => StatusCode::BAD_GATEWAY,
            Self::Network(_) => StatusCode::BAD_GATEWAY,
            Self::Parse { .. } => StatusCode::BAD_GATEWAY,
            Self::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    /// Parse a reqwest Response into the expected type T
    pub async fn parse_response<T: DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T, SupabaseError> {
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Self::http(status, body));
        }

        let body_text = response.text().await.map_err(Self::from_reqwest)?;
        serde_json::from_str::<T>(&body_text).map_err(|_| Self::parse(body_text))
    }
}

impl fmt::Display for SupabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http { status, body } => write!(f, "Supabase HTTP {} - {}", status, body),
            Self::Network(e) => write!(f, "Supabase network error: {}", e),
            Self::Parse { body } => write!(f, "Supabase parse error, body: {}", body),
            Self::Timeout(e) => write!(f, "Supabase timeout: {}", e),
        }
    }
}

impl std::error::Error for SupabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Network(e) | Self::Timeout(e) => Some(e),
            Self::Http { .. } | Self::Parse { .. } => None,
        }
    }
}
