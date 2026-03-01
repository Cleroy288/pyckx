//! Shared utilities and constants used across the application.
//!
//! This module contains truly generic utilities that have no
//! feature-specific dependencies.

pub mod constants;
pub mod utils;

pub use constants::{AppError, AppResult};
