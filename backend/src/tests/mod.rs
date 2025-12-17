//! Integration tests for LAPP backend
//!
//! # Structure
//! - `intello/` - Intello app tests (QCM CRUD, validation, ownership, property-based)
//! - `collection/` - Collection and DVD tests (lifecycle, edge cases)
//! - `apps/` - App and user app tests (CRUD, registry)
//! - `auth/` - Authentication tests (login, register)
//! - `utils/` - Utility tests (document extractor)

mod apps;
mod auth;
mod collection;
mod intello;
mod utils;
