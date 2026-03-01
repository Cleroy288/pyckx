//! Rate limit configuration - Defines limits per time window

// == RATE LIMIT CONFIG // ==

/// Configuration for rate limits on a specific route
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Max requests per second (None = unlimited)
    pub per_second: Option<u32>,
    /// Max requests per minute (None = unlimited)
    pub per_minute: Option<u32>,
    /// Max requests per hour (None = unlimited)
    pub per_hour: Option<u32>,
    /// Max requests per day (None = unlimited)
    pub per_day: Option<u32>,
}

impl RateLimitConfig {
    /// Create a new config with no limits
    pub fn new() -> Self {
        Self {
            per_second: None,
            per_minute: None,
            per_hour: None,
            per_day: None,
        }
    }

    /// Set requests per second limit
    pub fn per_second(mut self, limit: u32) -> Self {
        self.per_second = Some(limit);
        self
    }

    /// Set requests per minute limit
    pub fn per_minute(mut self, limit: u32) -> Self {
        self.per_minute = Some(limit);
        self
    }

    /// Set requests per hour limit
    pub fn per_hour(mut self, limit: u32) -> Self {
        self.per_hour = Some(limit);
        self
    }

    /// Set requests per day limit
    pub fn per_day(mut self, limit: u32) -> Self {
        self.per_day = Some(limit);
        self
    }

    /// Check if any limits are configured
    pub fn has_limits(&self) -> bool {
        self.per_second.is_some()
            || self.per_minute.is_some()
            || self.per_hour.is_some()
            || self.per_day.is_some()
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self::new()
    }
}

// == PRESET CONFIGURATIONS // ==

impl RateLimitConfig {
    /// Strict limits for sensitive endpoints (login, register)
    pub fn strict() -> Self {
        Self::new()
            .per_second(2)
            .per_minute(10)
            .per_hour(50)
            .per_day(200)
    }

    /// Standard limits for normal API endpoints
    pub fn standard() -> Self {
        Self::new().per_second(10).per_minute(100).per_hour(1000)
    }

    /// Relaxed limits for read-heavy endpoints
    pub fn relaxed() -> Self {
        Self::new().per_second(30).per_minute(300).per_hour(5000)
    }

    /// AI generation limits - 1 per minute to prevent abuse
    pub fn ai_generation() -> Self {
        Self::new().per_minute(10).per_hour(30).per_day(100)
    }
}

// == UNIT TESTS // ==

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = RateLimitConfig::new().per_second(5).per_minute(50);

        assert_eq!(config.per_second, Some(5));
        assert_eq!(config.per_minute, Some(50));
        assert_eq!(config.per_hour, None);
        assert_eq!(config.per_day, None);
    }

    #[test]
    fn test_has_limits() {
        let empty = RateLimitConfig::new();
        assert!(!empty.has_limits());

        let with_limit = RateLimitConfig::new().per_second(10);
        assert!(with_limit.has_limits());
    }

    #[test]
    fn test_presets() {
        let strict = RateLimitConfig::strict();
        assert_eq!(strict.per_second, Some(2));

        let standard = RateLimitConfig::standard();
        assert_eq!(standard.per_second, Some(10));
    }

    #[test]
    fn test_new_returns_all_none() {
        // arrange / act
        let config = RateLimitConfig::new();

        // assert
        assert_eq!(config.per_second, None);
        assert_eq!(config.per_minute, None);
        assert_eq!(config.per_hour, None);
        assert_eq!(config.per_day, None);
    }

    #[test]
    fn test_default_equals_new() {
        // arrange / act
        let config = RateLimitConfig::default();

        // assert
        assert!(!config.has_limits());
    }

    #[test]
    fn test_per_second_sets_value() {
        // arrange / act
        let config = RateLimitConfig::new().per_second(42);

        // assert
        assert_eq!(config.per_second, Some(42));
    }

    #[test]
    fn test_per_minute_sets_value() {
        // arrange / act
        let config = RateLimitConfig::new().per_minute(99);

        // assert
        assert_eq!(config.per_minute, Some(99));
    }

    #[test]
    fn test_per_hour_sets_value() {
        // arrange / act
        let config = RateLimitConfig::new().per_hour(500);

        // assert
        assert_eq!(config.per_hour, Some(500));
    }

    #[test]
    fn test_per_day_sets_value() {
        // arrange / act
        let config = RateLimitConfig::new().per_day(1000);

        // assert
        assert_eq!(config.per_day, Some(1000));
    }

    #[test]
    fn test_has_limits_with_only_minute_returns_true() {
        // arrange / act
        let config = RateLimitConfig::new().per_minute(10);

        // assert
        assert!(config.has_limits());
    }

    #[test]
    fn test_has_limits_with_only_hour_returns_true() {
        // arrange / act
        let config = RateLimitConfig::new().per_hour(100);

        // assert
        assert!(config.has_limits());
    }

    #[test]
    fn test_has_limits_with_only_day_returns_true() {
        // arrange / act
        let config = RateLimitConfig::new().per_day(500);

        // assert
        assert!(config.has_limits());
    }

    #[test]
    fn test_strict_preset_all_fields() {
        // arrange / act
        let config = RateLimitConfig::strict();

        // assert
        assert_eq!(config.per_second, Some(2));
        assert_eq!(config.per_minute, Some(10));
        assert_eq!(config.per_hour, Some(50));
        assert_eq!(config.per_day, Some(200));
    }

    #[test]
    fn test_standard_preset_all_fields() {
        // arrange / act
        let config = RateLimitConfig::standard();

        // assert
        assert_eq!(config.per_second, Some(10));
        assert_eq!(config.per_minute, Some(100));
        assert_eq!(config.per_hour, Some(1000));
        assert_eq!(config.per_day, None);
    }

    #[test]
    fn test_relaxed_preset_all_fields() {
        // arrange / act
        let config = RateLimitConfig::relaxed();

        // assert
        assert_eq!(config.per_second, Some(30));
        assert_eq!(config.per_minute, Some(300));
        assert_eq!(config.per_hour, Some(5000));
        assert_eq!(config.per_day, None);
    }

    #[test]
    fn test_ai_generation_preset_all_fields() {
        // arrange / act
        let config = RateLimitConfig::ai_generation();

        // assert
        assert_eq!(config.per_second, None);
        assert_eq!(config.per_minute, Some(10));
        assert_eq!(config.per_hour, Some(30));
        assert_eq!(config.per_day, Some(100));
    }

    #[test]
    fn test_builder_chain_all_fields() {
        // arrange / act
        let config = RateLimitConfig::new()
            .per_second(1)
            .per_minute(2)
            .per_hour(3)
            .per_day(4);

        // assert
        assert_eq!(config.per_second, Some(1));
        assert_eq!(config.per_minute, Some(2));
        assert_eq!(config.per_hour, Some(3));
        assert_eq!(config.per_day, Some(4));
    }
}
