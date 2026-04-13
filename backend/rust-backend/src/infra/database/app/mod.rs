//! App repository module
//!
//! Defines the contract for managing available apps in the platform.

mod r#trait;
mod types;

pub use r#trait::AppRepository;
pub use types::{CreateApp, UpdateApp};
