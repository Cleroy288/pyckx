//! Integration tests for LAPP backend
//!
//! # Structure
//! - `intello/` - Intello app tests (QCM CRUD, validation, ownership, property-based)
//! - `collection/` - Collection and DVD tests (lifecycle, edge cases)
//! - `app_registry/` - App and user app tests (CRUD, registry)
//! - `auth/` - Authentication tests (login, register)
//! - `utils/` - Utility tests (document extractor)
//! - `openrouter/` - OpenRouter service tests (JSON extraction, sanitization, prompt builder)
//! - `config/` - Test configuration

mod app_registry;
mod auth;
mod collection;
mod intello;
mod openrouter;
mod utils;
pub mod config;
