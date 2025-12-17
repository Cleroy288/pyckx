//! Collection test helpers

use crate::config::Config;
use crate::infrastructure::CreateDvd;
use chrono::{TimeZone, Utc};

/// Load test configuration from environment
pub fn load_test_config() -> Option<Config> {
    dotenv::dotenv().ok();
    Config::from_env().ok()
}

/// Create a test DVD for testing
pub fn create_test_dvd(name: &str, user_id: &str, collection_id: i32) -> CreateDvd {
    let year = Utc.with_ymd_and_hms(2010, 7, 16, 0, 0, 0).unwrap();
    CreateDvd::new(
        name,
        year,
        Some("Christopher Nolan".to_string()),
        vec!["Leonardo DiCaprio".to_string(), "Tom Hardy".to_string()],
        Some("Sci-Fi".to_string()),
        user_id,
        collection_id,
    )
}
