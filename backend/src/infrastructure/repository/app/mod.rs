//! App repository module
//!
//! Defines the contract for managing available apps in the platform.

mod types;
mod r#trait;

pub use types::{CreateApp, UpdateApp};
pub use r#trait::AppRepository;
