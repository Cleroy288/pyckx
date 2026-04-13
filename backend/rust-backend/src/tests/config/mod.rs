//! Test configuration - Credentials for integration tests
//!
//! This module provides test-specific configuration that should
//! NEVER be used in production code. It loads test credentials
//! from environment variables.

use crate::configs::error::{required_env, ConfigError};

// == TEST CREDENTIALS CONFIGURATION // ==

/// Test credentials loaded from environment variables.
/// Only used in `#[cfg(test)]` contexts.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestConfig {
    /// Test user email
    pub email: String,
    /// Test user password
    pub password: String,
}

impl TestConfig {
    /// Load test configuration from environment.
    ///
    /// # Returns
    /// - `Ok(TestConfig)` if credentials are present
    /// - `Err(ConfigError)` if any credential is missing
    ///
    /// # Example
    /// ```ignore
    /// let test_cfg = TestConfig::from_env()?;
    /// let email = &test_cfg.email;
    /// ```
    #[allow(dead_code)]
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv::from_path(std::path::Path::new("../.env")).ok();

        Ok(Self {
            email: required_env("TEST_EMAIL")?,
            password: required_env("TEST_PSSWRD")?,
        })
    }
}

// == UNIT TESTS // ==

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads_from_env() {
        // This test validates that env vars are set correctly
        let result = TestConfig::from_env();
        assert!(result.is_ok(), "TestConfig should load from .env");

        let cfg = result.unwrap();
        assert!(!cfg.email.is_empty(), "TEST_EMAIL should not be empty");
        assert!(!cfg.password.is_empty(), "TEST_PSSWRD should not be empty");
    }
}
