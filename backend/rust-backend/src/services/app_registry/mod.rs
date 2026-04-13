//! App Registry Service - App management and registry
//!
//! # Architecture
//! - Domain: `registry_domain.rs`, `error_domain.rs`
//! - Service: `registry_service.rs`

// Domain modules
pub mod error_domain;
pub mod registry_domain;

// Service logic
pub mod registry_service;

// Re-exports
pub use error_domain::AppError as AppsError;
pub use registry_service::AppService;

// Re-export app types from their modules (kept for compatibility if needed, else remove)
pub use crate::services::collection::app_domain::CollectionApp;
pub use crate::services::StudyApp;
