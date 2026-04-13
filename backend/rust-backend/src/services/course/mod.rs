//! Course module - Comprehensive course generation and management
//!
//! This module contains all course-related functionality organized by:
//! - domain: Entities, inputs, and outputs
//! - prompt: AI prompt construction
//! - parser: AI response parsing
//! - service: Business logic (CRUD, generation, resources)

pub mod domain;
pub mod parser;
pub mod prompt;
pub mod service;

// Re-export commonly used types for convenience
