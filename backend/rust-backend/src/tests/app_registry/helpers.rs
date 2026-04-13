//! Apps test helpers

use crate::configs::Config;

/// Load test configuration from environment
pub fn load_test_config() -> Option<Config> {
    dotenv::from_path(std::path::Path::new("../.env")).ok();
    Config::from_env().ok()
}
