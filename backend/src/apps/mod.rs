//! Apps module - Contains all app implementations
//!
//! Each app is a self-contained feature module with its own
//! domain entities, services, and API handlers.
//!
//! # Architecture
//! - Domain entities: `domain/{app_name}/`
//! - Business logic: `services/{app_name}_service.rs`
//! - Storage: `infrastructure/` (repository implementations)
//!
//! This module exports only app metadata and constants.
//!
//! # App Registry
//! The `registry` module contains the central list of all available apps.
//! Use `is_valid_app()` to validate app names before operations.

pub mod collection;
pub mod intello;
pub mod registry;

pub use collection::CollectionApp;
pub use intello::IntelloApp;
