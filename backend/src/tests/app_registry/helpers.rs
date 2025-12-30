//! Apps test helpers

use crate::configs::Config;

/// Load test configuration from environment
pub fn load_test_config() -> Option<Config> {
    dotenv::dotenv().ok();
    Config::from_env().ok()
}
