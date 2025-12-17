//! Rate limiter store - Thread-safe storage for request counts

use super::config::RateLimitConfig;
use super::error::RateLimitError;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// == TIME WINDOWS // ==

/// Time window durations
const SECOND: Duration = Duration::from_secs(1);
const MINUTE: Duration = Duration::from_secs(60);
const HOUR: Duration = Duration::from_secs(3600);
const DAY: Duration = Duration::from_secs(86400);

// == BUCKET ENTRY // ==

/// Tracks request count within a time window
#[derive(Debug, Clone)]
struct Bucket {
    count: u32,
    window_start: Instant,
    window_duration: Duration,
}

impl Bucket {
    fn new(duration: Duration) -> Self {
        Self {
            count: 0,
            window_start: Instant::now(),
            window_duration: duration,
        }
    }

    /// Increment count, resetting if window expired
    fn increment(&mut self) -> u32 {
        let now = Instant::now();
        if now.duration_since(self.window_start) >= self.window_duration {
            // Window expired, reset
            self.count = 1;
            self.window_start = now;
        } else {
            self.count += 1;
        }
        self.count
    }

    /// Get current count (reset if window expired)
    fn current_count(&self) -> u32 {
        let now = Instant::now();
        if now.duration_since(self.window_start) >= self.window_duration {
            0
        } else {
            self.count
        }
    }

    /// Time until window resets
    fn time_until_reset(&self) -> Duration {
        let elapsed = Instant::now().duration_since(self.window_start);
        if elapsed >= self.window_duration {
            Duration::ZERO
        } else {
            self.window_duration - elapsed
        }
    }
}

// == CLIENT ENTRY // ==

/// Rate limit state for a single client (IP or user)
#[derive(Debug)]
struct ClientEntry {
    second: Option<Bucket>,
    minute: Option<Bucket>,
    hour: Option<Bucket>,
    day: Option<Bucket>,
}

impl ClientEntry {
    fn new(config: &RateLimitConfig) -> Self {
        Self {
            second: config.per_second.map(|_| Bucket::new(SECOND)),
            minute: config.per_minute.map(|_| Bucket::new(MINUTE)),
            hour: config.per_hour.map(|_| Bucket::new(HOUR)),
            day: config.per_day.map(|_| Bucket::new(DAY)),
        }
    }
}

// == RATE LIMITER // ==

/// Thread-safe rate limiter with per-route configuration
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Route configurations: path -> config
    configs: Arc<DashMap<String, RateLimitConfig>>,
    /// Client state: (route, client_key) -> entry
    entries: Arc<DashMap<(String, String), ClientEntry>>,
    /// Default config for unconfigured routes
    default_config: Option<RateLimitConfig>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new() -> Self {
        Self {
            configs: Arc::new(DashMap::new()),
            entries: Arc::new(DashMap::new()),
            default_config: None,
        }
    }

    /// Create with a default config for all routes
    pub fn with_default(config: RateLimitConfig) -> Self {
        Self {
            configs: Arc::new(DashMap::new()),
            entries: Arc::new(DashMap::new()),
            default_config: Some(config),
        }
    }

    /// Configure rate limits for a specific route
    pub fn configure(&self, route: impl Into<String>, config: RateLimitConfig) {
        self.configs.insert(route.into(), config);
    }

    /// Configure multiple routes with the same config
    pub fn configure_many(&self, routes: &[&str], config: RateLimitConfig) {
        for route in routes {
            self.configs.insert((*route).to_string(), config.clone());
        }
    }

    /// Get config for a route (falls back to default)
    fn get_config(&self, route: &str) -> Option<RateLimitConfig> {
        self.configs
            .get(route)
            .map(|r| r.clone())
            .or_else(|| self.default_config.clone())
    }

    /// Check if request is allowed, returns error if rate limited
    ///
    /// # Arguments
    /// * `route` - The request path
    /// * `client_key` - Unique client identifier (IP address or user ID)
    pub fn check(&self, route: &str, client_key: &str) -> Result<(), RateLimitError> {
        let config = match self.get_config(route) {
            Some(c) if c.has_limits() => c,
            _ => return Ok(()), // No limits configured
        };

        let key = (route.to_string(), client_key.to_string());

        // Get or create entry
        let mut entry = self
            .entries
            .entry(key)
            .or_insert_with(|| ClientEntry::new(&config));

        // Check each window
        if let (Some(limit), Some(bucket)) = (config.per_second, entry.second.as_mut()) {
            let count = bucket.increment();
            if count > limit {
                return Err(RateLimitError::new(
                    "second",
                    limit,
                    bucket.time_until_reset(),
                ));
            }
        }

        if let (Some(limit), Some(bucket)) = (config.per_minute, entry.minute.as_mut()) {
            let count = bucket.increment();
            if count > limit {
                return Err(RateLimitError::new(
                    "minute",
                    limit,
                    bucket.time_until_reset(),
                ));
            }
        }

        if let (Some(limit), Some(bucket)) = (config.per_hour, entry.hour.as_mut()) {
            let count = bucket.increment();
            if count > limit {
                return Err(RateLimitError::new(
                    "hour",
                    limit,
                    bucket.time_until_reset(),
                ));
            }
        }

        if let (Some(limit), Some(bucket)) = (config.per_day, entry.day.as_mut()) {
            let count = bucket.increment();
            if count > limit {
                return Err(RateLimitError::new("day", limit, bucket.time_until_reset()));
            }
        }

        Ok(())
    }

    /// Get remaining requests for a client on a route
    #[allow(dead_code)]
    pub fn remaining(&self, route: &str, client_key: &str) -> RemainingLimits {
        let config = self.get_config(route);
        let key = (route.to_string(), client_key.to_string());

        let entry = self.entries.get(&key);

        RemainingLimits {
            per_second: config.as_ref().and_then(|c| c.per_second).map(|limit| {
                let used = entry
                    .as_ref()
                    .and_then(|e| e.second.as_ref())
                    .map(|b| b.current_count())
                    .unwrap_or(0);
                limit.saturating_sub(used)
            }),
            per_minute: config.as_ref().and_then(|c| c.per_minute).map(|limit| {
                let used = entry
                    .as_ref()
                    .and_then(|e| e.minute.as_ref())
                    .map(|b| b.current_count())
                    .unwrap_or(0);
                limit.saturating_sub(used)
            }),
            per_hour: config.as_ref().and_then(|c| c.per_hour).map(|limit| {
                let used = entry
                    .as_ref()
                    .and_then(|e| e.hour.as_ref())
                    .map(|b| b.current_count())
                    .unwrap_or(0);
                limit.saturating_sub(used)
            }),
            per_day: config.as_ref().and_then(|c| c.per_day).map(|limit| {
                let used = entry
                    .as_ref()
                    .and_then(|e| e.day.as_ref())
                    .map(|b| b.current_count())
                    .unwrap_or(0);
                limit.saturating_sub(used)
            }),
        }
    }

    /// Clean up expired entries (call periodically)
    #[allow(dead_code)]
    pub fn cleanup(&self) {
        self.entries.retain(|_, entry| {
            // Keep if any bucket is still active
            let second_active = entry
                .second
                .as_ref()
                .map(|b| b.current_count() > 0)
                .unwrap_or(false);
            let minute_active = entry
                .minute
                .as_ref()
                .map(|b| b.current_count() > 0)
                .unwrap_or(false);
            let hour_active = entry
                .hour
                .as_ref()
                .map(|b| b.current_count() > 0)
                .unwrap_or(false);
            let day_active = entry
                .day
                .as_ref()
                .map(|b| b.current_count() > 0)
                .unwrap_or(false);

            second_active || minute_active || hour_active || day_active
        });
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

// == REMAINING LIMITS // ==

/// Remaining request counts for a client
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RemainingLimits {
    pub per_second: Option<u32>,
    pub per_minute: Option<u32>,
    pub per_hour: Option<u32>,
    pub per_day: Option<u32>,
}


// == UNIT TESTS // ==

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_increment() {
        let mut bucket = Bucket::new(Duration::from_secs(1));
        assert_eq!(bucket.increment(), 1);
        assert_eq!(bucket.increment(), 2);
        assert_eq!(bucket.increment(), 3);
    }

    #[test]
    fn test_bucket_reset() {
        let mut bucket = Bucket::new(Duration::from_millis(50));
        bucket.increment();
        bucket.increment();

        std::thread::sleep(Duration::from_millis(60));

        // Should reset after window expires
        assert_eq!(bucket.increment(), 1);
    }

    #[test]
    fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new();
        limiter.configure("/test", RateLimitConfig::new().per_second(5));

        for _ in 0..5 {
            assert!(limiter.check("/test", "client1").is_ok());
        }
    }

    #[test]
    fn test_rate_limiter_blocks_over_limit() {
        let limiter = RateLimiter::new();
        limiter.configure("/test", RateLimitConfig::new().per_second(3));

        // First 3 should pass
        for _ in 0..3 {
            assert!(limiter.check("/test", "client1").is_ok());
        }

        // 4th should fail
        let result = limiter.check("/test", "client1");
        assert!(result.is_err());
    }

    #[test]
    fn test_rate_limiter_separate_clients() {
        let limiter = RateLimiter::new();
        limiter.configure("/test", RateLimitConfig::new().per_second(2));

        // Client 1
        assert!(limiter.check("/test", "client1").is_ok());
        assert!(limiter.check("/test", "client1").is_ok());
        assert!(limiter.check("/test", "client1").is_err());

        // Client 2 should still have quota
        assert!(limiter.check("/test", "client2").is_ok());
        assert!(limiter.check("/test", "client2").is_ok());
    }

    #[test]
    fn test_rate_limiter_separate_routes() {
        let limiter = RateLimiter::new();
        limiter.configure("/route1", RateLimitConfig::new().per_second(2));
        limiter.configure("/route2", RateLimitConfig::new().per_second(2));

        // Exhaust route1
        limiter.check("/route1", "client1").unwrap();
        limiter.check("/route1", "client1").unwrap();
        assert!(limiter.check("/route1", "client1").is_err());

        // Route2 should still work
        assert!(limiter.check("/route2", "client1").is_ok());
    }

    #[test]
    fn test_unconfigured_route_allows_all() {
        let limiter = RateLimiter::new();
        // No config for /unconfigured

        for _ in 0..100 {
            assert!(limiter.check("/unconfigured", "client1").is_ok());
        }
    }

    #[test]
    fn test_default_config() {
        let limiter = RateLimiter::with_default(RateLimitConfig::new().per_second(2));

        // Any route should use default
        assert!(limiter.check("/any", "client1").is_ok());
        assert!(limiter.check("/any", "client1").is_ok());
        assert!(limiter.check("/any", "client1").is_err());
    }

    #[test]
    fn test_multiple_windows() {
        let limiter = RateLimiter::new();
        limiter.configure(
            "/test",
            RateLimitConfig::new().per_second(10).per_minute(5),
        );

        // Should hit minute limit before second limit
        for _ in 0..5 {
            assert!(limiter.check("/test", "client1").is_ok());
        }

        // 6th should fail (minute limit)
        let result = limiter.check("/test", "client1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().window, "minute");
    }
}
