//! Rate limit error types

use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use std::time::Duration;

// == RATE LIMIT ERROR // ==

/// Error returned when rate limit is exceeded
#[derive(Debug, Clone)]
pub struct RateLimitError {
    /// Which window was exceeded (second, minute, hour, day)
    pub window: &'static str,
    /// The limit that was exceeded
    pub limit: u32,
    /// Time until the window resets
    pub retry_after: Duration,
}

impl RateLimitError {
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
            + if self.retry_after.subsec_nanos() > 0 {
                1
            } else {
                0
            }
    }
}

impl fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rate limit exceeded: {} requests per {} (retry after {} seconds)",
            self.limit,
            self.window,
            self.retry_after_secs()
        )
    }
}

impl std::error::Error for RateLimitError {}

impl ResponseError for RateLimitError {
    fn status_code(&self) -> StatusCode {
        StatusCode::TOO_MANY_REQUESTS
    }

    fn error_response(&self) -> HttpResponse {
        let retry_after = self.retry_after_secs().to_string();

        HttpResponse::build(self.status_code())
            .insert_header((
                HeaderName::from_static("retry-after"),
                HeaderValue::from_str(&retry_after)
                    .unwrap_or_else(|_| HeaderValue::from_static("60")),
            ))
            .insert_header((
                HeaderName::from_static("x-ratelimit-limit"),
                HeaderValue::from_str(&self.limit.to_string())
                    .unwrap_or_else(|_| HeaderValue::from_static("0")),
            ))
            .insert_header((
                HeaderName::from_static("x-ratelimit-remaining"),
                HeaderValue::from_static("0"),
            ))
            .insert_header((
                HeaderName::from_static("x-ratelimit-reset"),
                HeaderValue::from_str(&retry_after)
                    .unwrap_or_else(|_| HeaderValue::from_static("60")),
            ))
            .json(serde_json::json!({
                "code": "RATE_LIMIT_EXCEEDED",
                "message": format!("Too many requests. Limit: {} per {}.", self.limit, self.window),
                "retry_after_seconds": self.retry_after_secs(),
                "window": self.window
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stores_fields() {
        // arrange / act
        let err = RateLimitError::new(
            "minute",
            100,
            Duration::from_secs(30),
        );

        // assert
        assert_eq!(err.window, "minute");
        assert_eq!(err.limit, 100);
        assert_eq!(err.retry_after, Duration::from_secs(30));
    }

    #[test]
    fn test_retry_after_secs_exact_seconds() {
        // arrange
        let err = RateLimitError::new(
            "second",
            10,
            Duration::from_secs(5),
        );

        // act / assert
        assert_eq!(err.retry_after_secs(), 5);
    }

    #[test]
    fn test_retry_after_secs_rounds_up_with_nanos() {
        // arrange
        let err = RateLimitError::new(
            "second",
            10,
            Duration::from_millis(5500),
        );

        // act / assert (5.5s rounds up to 6)
        assert_eq!(err.retry_after_secs(), 6);
    }

    #[test]
    fn test_retry_after_secs_zero_duration() {
        // arrange
        let err = RateLimitError::new(
            "second",
            10,
            Duration::ZERO,
        );

        // act / assert
        assert_eq!(err.retry_after_secs(), 0);
    }

    #[test]
    fn test_display_format() {
        // arrange
        let err = RateLimitError::new(
            "minute",
            50,
            Duration::from_secs(30),
        );

        // act
        let display = err.to_string();

        // assert
        assert!(display.contains("50"));
        assert!(display.contains("minute"));
        assert!(display.contains("30"));
    }

    #[test]
    fn test_status_code_is_429() {
        // arrange
        let err = RateLimitError::new(
            "hour",
            100,
            Duration::from_secs(60),
        );

        // act / assert
        assert_eq!(err.status_code(), StatusCode::TOO_MANY_REQUESTS);
    }
}
