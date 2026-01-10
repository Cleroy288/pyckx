//! Configuration module - Environment configuration
//!
//! # Structure
//! - `config.rs` - Production configuration (server, Supabase)
//! - `error.rs` - Configuration error types
//!
//! # Example
//! ```ignore
//! use crate::configs::{Config, ConfigError};
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

pub use config::Config;
pub use error::ConfigError;
