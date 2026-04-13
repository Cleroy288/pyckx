//! Error module
//!
//! Minimal shared error infrastructure.
//! Service-specific errors live in their service folders.

mod global;

pub use global::{AppError, AppResult};
