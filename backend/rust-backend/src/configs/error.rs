//! Configuration error types
//!
//! Provides typed errors for configuration loading failures.

/// Errors that can occur when loading configuration
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Required environment variable is missing
    #[error(
        "Missing required environment variable: {var_name}"
    )]
    MissingEnvVar {
        /// Name of the missing variable
        var_name: &'static str,
    },
    /// Environment variable has invalid value
    #[allow(dead_code)]
    #[error(
        "Invalid value for {var_name}: \
         expected {expected}, got '{actual}'"
    )]
    InvalidValue {
        /// Name of the variable
        var_name: &'static str,
        /// Description of what was expected
        expected: &'static str,
        /// The actual value received
        actual: String,
    },
}

/// Extension trait for loading required env vars
pub trait EnvVarExt {
    /// Load a required environment variable
    fn required(
        var_name: &'static str,
    ) -> Result<String, ConfigError>;
}

impl EnvVarExt for std::env::VarError {
    fn required(
        var_name: &'static str,
    ) -> Result<String, ConfigError> {
        std::env::var(var_name)
            .map_err(|_| ConfigError::MissingEnvVar { var_name })
    }
}

/// Load a required environment variable
pub fn required_env(
    var_name: &'static str,
) -> Result<String, ConfigError> {
    std::env::VarError::required(var_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_scenarios() {
        let cases = vec![
            (
                ConfigError::MissingEnvVar {
                    var_name: "TEST_VAR",
                },
                "Missing required environment variable: \
                 TEST_VAR",
            ),
            (
                ConfigError::InvalidValue {
                    var_name: "PORT",
                    expected: "a valid port number (1-65535)",
                    actual: "not_a_number".to_string(),
                },
                "Invalid value for PORT: \
                 expected a valid port number (1-65535), \
                 got 'not_a_number'",
            ),
            (
                ConfigError::InvalidValue {
                    var_name: "LOG_LEVEL",
                    expected: "debug|info|warn|error",
                    actual: "verbose".to_string(),
                },
                "Invalid value for LOG_LEVEL: \
                 expected debug|info|warn|error, \
                 got 'verbose'",
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

    #[test]
    fn test_required_env_missing_var_returns_error() {
        let var = "LAPP_TEST_NONEXISTENT_VAR_XYZ";
        let result = required_env(var);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Missing required environment variable: \
             LAPP_TEST_NONEXISTENT_VAR_XYZ"
        );
    }

    #[test]
    fn test_required_env_existing_var_returns_value() {
        let result = required_env("PATH");
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
