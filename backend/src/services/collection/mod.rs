//! Collection Service Module
//! 
//! Service for managing user collections (DVDs, Books, etc.).
//!
//! # Architecture
//! - Domain entities: `*_domain.rs` files
//! - Business logic: `*_service.rs` files

// Domain modules
pub mod app_domain;
pub mod collection_domain;
pub mod dvd_domain;
pub mod error_domain;
mod types_domain;

// Service modules
mod collection_service;
mod dvd_service;

pub use types_domain::CollectionService;

