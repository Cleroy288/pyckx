//! Configuration module - Environment and test configuration
//!
//! # Structure
//! - `config.rs` - Production configuration (server, Supabase)
//! - `test_config.rs` - Test credentials (only for tests)
//! - `error.rs` - Configuration error types
//!
//! # Example
//! ```ignore
//! use crate::config::{Config, ConfigError};
//!
//! fn main() {
//!     let config = Config::from_env().unwrap_or_else(|e| {
//!         eprintln!("Configuration error: {}", e);
//!         std::process::exit(1);
//!     });
//! }
//! ```

#[allow(clippy::module_inception)]
pub mod config;
pub mod error;
pub mod test_config;

pub use config::Config;
pub use error::ConfigError;

// TestConfig is only used in tests
#[cfg(test)]
pub use test_config::TestConfig;
