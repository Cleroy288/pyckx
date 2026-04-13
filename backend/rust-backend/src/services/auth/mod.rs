//! Auth Service - Authentication and session management
//!
//! # Architecture
//! - Domain: `error_domain.rs`, `types_domain.rs`
//! - Service: `auth_service.rs`

// Domain modules
pub mod error_domain;
pub mod types_domain;

// Service logic
mod auth_service;

// Re-exports
pub use types_domain::AuthService;
