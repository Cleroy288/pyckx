//! Application configuration - Environment variables
//!
//! Loads configuration from `.env` file with proper error handling.
//! Returns `Result<Config, ConfigError>` instead of panicking.

use dotenv::dotenv;
use std::fmt;
use tracing::info;

use super::error::{required_env, ConfigError};

// == APPLICATION CONFIGURATION // ==

/// Application configuration loaded from environment
#[derive(Debug, Clone)]
pub struct Config {
    /// Server IP address
    pub ip: String,
    /// Server port
    pub port: String,
    /// Supabase URL
    pub sp_url: String,
    /// Supabase anonymous key
    pub sp_anon: String,
    /// Supabase service role key
    pub sp_service_role: String,
    /// Whether to use HTTPS
    pub secure_http: String,
    /// Test user ID for integration tests (optional)
    #[allow(dead_code)]
    pub test_user_id: Option<String>,
    /// Whether to serve frontend static files (production mode)
    pub serve_frontend: bool,
    /// Directory containing frontend static files
    pub static_dir: String,
    /// OpenRouter API key for AI services
    pub openrouter_api_key: Option<String>,
    /// Google AI API key for Gemini models (optional, for higher rate limits)
    pub google_ai_key: Option<String>,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Returns
    /// - `Ok(Config)` if all required variables are present
    /// - `Err(ConfigError)` if any required variable is missing
    ///
    /// # Required Environment Variables
    /// - `IP` - Server IP address
    /// - `PORT` - Server port
    /// - `SP_URL` - Supabase URL
    /// - `SP_ANON` - Supabase anonymous key
    /// - `SP_SERVICE_ROLE` - Supabase service role key
    /// - `SECURE_HTTP` - Whether to use HTTPS ("true" or "false")
    ///
    /// # Example
    /// ```ignore
    /// let config = Config::from_env()?;
    /// println!("Server running on {}:{}", config.ip, config.port);
    /// ```
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file (ignore if not present)
        dotenv().ok();

        // == LOAD REQUIRED ENVIRONMENT VARIABLES // ==
        let config = Self {
            ip: required_env("IP")?,
            port: required_env("PORT")?,
            sp_url: required_env("SP_URL")?,
            sp_anon: required_env("SP_ANON")?,
            sp_service_role: required_env("SP_SERVICE_ROLE")?,
            secure_http: required_env("SECURE_HTTP")?,
            // Optional: Test user ID for integration tests
            test_user_id: std::env::var("TEST_USR_ID")
                .ok()
                .filter(|s| !s.is_empty()),
            // Frontend serving configuration (defaults for development)
            serve_frontend: std::env::var("SERVE_FRONTEND")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false),
            static_dir: std::env::var("STATIC_DIR")
                .unwrap_or_else(|_| "./static".to_string()),
            // OpenRouter API key (optional)
            openrouter_api_key: std::env::var("OPENROUTER_API_KEY")
                .ok()
                .filter(|s| !s.is_empty()),
            // Google AI API key for Gemini models (optional, for higher rate limits)
            google_ai_key: std::env::var("GOOGLE_AI_KEY")
                .ok()
                .filter(|s| !s.is_empty()),
        };

        info!(
            ip = %config.ip,
            port = %config.port,
            supabase_url = %config.sp_url,
            "Configuration loaded"
        );

        Ok(config)
    }

    /// Get test user ID if configured
    #[allow(dead_code)]
    pub fn get_test_user_id(&self) -> Option<&str> {
        self.test_user_id.as_deref()
    }
}

// == DISPLAY IMPLEMENTATION // ==

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config({}:{})", self.ip, self.port)
    }
}

// == UNIT TESTS // ==

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env_loads_successfully() {
        // This test requires .env to be properly configured
        let result = Config::from_env();
        assert!(result.is_ok(), "Config should load from .env");
    }

    #[test]
    fn test_config_display() {
        let config = Config {
            ip: "127.0.0.1".to_string(),
            port: "8080".to_string(),
            sp_url: "https://test.supabase.co".to_string(),
            sp_anon: "anon".to_string(),
            sp_service_role: "service".to_string(),
            secure_http: "true".to_string(),
            test_user_id: Some("test-user-123".to_string()),
            serve_frontend: false,
            static_dir: "./static".to_string(),
            openrouter_api_key: None,
            google_ai_key: None,
        };
        assert_eq!(config.to_string(), "Config(127.0.0.1:8080)");
    }

    #[test]
    fn test_get_test_user_id() {
        let config = Config {
            ip: "127.0.0.1".to_string(),
            port: "8080".to_string(),
            sp_url: "https://test.supabase.co".to_string(),
            sp_anon: "anon".to_string(),
            sp_service_role: "service".to_string(),
            secure_http: "true".to_string(),
            test_user_id: Some("user-uuid-123".to_string()),
            serve_frontend: false,
            static_dir: "./static".to_string(),
            openrouter_api_key: None,
            google_ai_key: None,
        };
        assert_eq!(config.get_test_user_id(), Some("user-uuid-123"));
    }

    #[test]
    fn test_get_test_user_id_none_when_not_set() {
        // arrange
        let config = Config {
            ip: "0.0.0.0".to_string(),
            port: "3000".to_string(),
            sp_url: "https://x.supabase.co".to_string(),
            sp_anon: "anon".to_string(),
            sp_service_role: "srv".to_string(),
            secure_http: "false".to_string(),
            test_user_id: None,
            serve_frontend: false,
            static_dir: "./static".to_string(),
            openrouter_api_key: None,
            google_ai_key: None,
        };

        // act / assert
        assert_eq!(config.get_test_user_id(), None);
    }
}
