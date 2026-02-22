//! Configuration error types
//!
//! Provides typed errors for configuration loading failures.

use std::fmt;

// == CONFIG ERROR TYPE // ==

/// Errors that can occur when loading configuration
#[derive(Debug)]
pub enum ConfigError {
    /// Required environment variable is missing
    MissingEnvVar {
        /// Name of the missing variable
        var_name: &'static str,
    },
    /// Environment variable has invalid value (reserved for future validation)
    #[allow(dead_code)]
    InvalidValue {
        /// Name of the variable
        var_name: &'static str,
        /// Description of what was expected
        expected: &'static str,
        /// The actual value received
        actual: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnvVar { var_name } => {
                write!(f, "Missing required environment variable: {}", var_name)
            }
            Self::InvalidValue {
                var_name,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Invalid value for {}: expected {}, got '{}'",
                    var_name, expected, actual
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

// == HELPER TRAIT FOR ENV VAR LOADING // ==

/// Extension trait for loading required env vars with proper errors
pub trait EnvVarExt {
    /// Load a required environment variable
    fn required(var_name: &'static str) -> Result<String, ConfigError>;
}

impl EnvVarExt for std::env::VarError {
    fn required(var_name: &'static str) -> Result<String, ConfigError> {
        std::env::var(var_name)
            .map_err(|_| ConfigError::MissingEnvVar { var_name })
    }
}

// == CONVENIENCE FUNCTIONS // ==

/// Load a required environment variable
pub fn required_env(var_name: &'static str) -> Result<String, ConfigError> {
    std::env::VarError::required(var_name)
}

// == UNIT TESTS // ==

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_env_var_error_display() {
        let err = ConfigError::MissingEnvVar {
            var_name: "TEST_VAR",
        };
        assert_eq!(
            err.to_string(),
            "Missing required environment variable: TEST_VAR"
        );
    }

    #[test]
    fn test_invalid_value_error_display() {
        let err = ConfigError::InvalidValue {
            var_name: "PORT",
            expected: "a valid port number (1-65535)",
            actual: "not_a_number".to_string(),
        };
        assert!(err.to_string().contains("PORT"));
        assert!(err.to_string().contains("not_a_number"));
    }

    #[test]
    fn test_required_env_missing_var_returns_error() {
        // arrange - use a var name that almost certainly doesn't exist
        let var_name = "LAPP_TEST_NONEXISTENT_VAR_XYZ";

        // act
        let result = required_env(var_name);

        // assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Missing required environment variable: LAPP_TEST_NONEXISTENT_VAR_XYZ"
        );
    }

    #[test]
    fn test_required_env_existing_var_returns_value() {
        // arrange - PATH should always exist
        let result = required_env("PATH");

        // assert
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_invalid_value_display_format() {
        // arrange
        let err = ConfigError::InvalidValue {
            var_name: "LOG_LEVEL",
            expected: "debug|info|warn|error",
            actual: "verbose".to_string(),
        };

        // act
        let display = err.to_string();

        // assert
        assert_eq!(
            display,
            "Invalid value for LOG_LEVEL: expected debug|info|warn|error, got 'verbose'"
        );
    }
}
