//! Rate limit error types

use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::time::Duration;

/// Error returned when rate limit is exceeded
#[derive(Debug, Clone, thiserror::Error)]
#[error(
    "Rate limit exceeded: {limit} requests per {window} \
     (retry after {} seconds)",
    self.retry_after_secs()
)]
pub struct RateLimitError {
    /// Which window was exceeded
    pub window: &'static str,
    /// The limit that was exceeded
    pub limit: u32,
    /// Time until the window resets
    pub retry_after: Duration,
}

impl RateLimitError {
    /// Create a new rate limit error
    pub fn new(
        window: &'static str,
        limit: u32,
        retry_after: Duration,
    ) -> Self {
        Self {
            window,
            limit,
            retry_after,
        }
    }

    /// Get retry-after in seconds (rounded up)
    pub fn retry_after_secs(&self) -> u64 {
        self.retry_after.as_secs()
            + u64::from(
                self.retry_after.subsec_nanos() > 0,
            )
    }
}

impl ResponseError for RateLimitError {
    fn status_code(&self) -> StatusCode {
        StatusCode::TOO_MANY_REQUESTS
    }

    fn error_response(&self) -> HttpResponse {
        let retry = self.retry_after_secs().to_string();
        let limit_str = self.limit.to_string();
        let fallback = HeaderValue::from_static("60");

        HttpResponse::build(self.status_code())
            .insert_header((
                HeaderName::from_static("retry-after"),
                HeaderValue::from_str(&retry)
                    .unwrap_or_else(|_| fallback.clone()),
            ))
            .insert_header((
                HeaderName::from_static(
                    "x-ratelimit-limit",
                ),
                HeaderValue::from_str(&limit_str)
                    .unwrap_or_else(|_| {
                        HeaderValue::from_static("0")
                    }),
            ))
            .insert_header((
                HeaderName::from_static(
                    "x-ratelimit-remaining",
                ),
                HeaderValue::from_static("0"),
            ))
            .insert_header((
                HeaderName::from_static(
                    "x-ratelimit-reset",
                ),
                HeaderValue::from_str(&retry)
                    .unwrap_or(fallback),
            ))
            .json(serde_json::json!({
                "code": "RATE_LIMIT_EXCEEDED",
                "message": format!(
                    "Too many requests. \
                     Limit: {} per {}.",
                    self.limit, self.window
                ),
                "retry_after_seconds":
                    self.retry_after_secs(),
                "window": self.window
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stores_fields() {
        let err = RateLimitError::new(
            "minute",
            100,
            Duration::from_secs(30),
        );
        assert_eq!(err.window, "minute");
        assert_eq!(err.limit, 100);
        assert_eq!(
            err.retry_after,
            Duration::from_secs(30)
        );
    }

    #[test]
    fn test_retry_after_secs_scenarios() {
        let cases = vec![
            (Duration::from_secs(5), 5u64),
            (Duration::from_millis(5500), 6u64),
            (Duration::ZERO, 0u64),
        ];
        for (duration, expected) in cases {
            let err =
                RateLimitError::new("second", 10, duration);
            assert_eq!(
                err.retry_after_secs(),
                expected,
                "retry_after_secs for {:?}",
                duration
            );
        }
    }

    #[test]
    fn test_display_format() {
        let err = RateLimitError::new(
            "minute",
            50,
            Duration::from_secs(30),
        );
        let display = err.to_string();
        assert!(display.contains("50"));
        assert!(display.contains("minute"));
        assert!(display.contains("30"));
    }

    #[test]
    fn test_status_code_is_429() {
        let err = RateLimitError::new(
            "hour",
            100,
            Duration::from_secs(60),
        );
        assert_eq!(
            err.status_code(),
            StatusCode::TOO_MANY_REQUESTS
        );
    }
}
