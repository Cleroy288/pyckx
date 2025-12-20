//! Use Cases layer - Application orchestration
//!
//! This module contains use cases that coordinate between services,
//! handle validation, and implement business workflows.
//!
//! ## Architecture
//!
//! ```text
//! Handler → Use Case → Service(s) → Repository
//! ```
//!
//! Use Cases are the entry point for business operations. They:
//! - Validate input parameters
//! - Coordinate multiple services
//! - Handle cross-cutting concerns
//! - Return domain results
//!
//! ## Available Use Case Modules
//!
//! - `intello` - Intello app operations (QCM, flashcards, etc.)
//! - `collection` - Collection app operations (DVDs, items)
//! - `ai` - Shared AI generation operations

pub mod ai;
pub mod collection;
pub mod intello;
