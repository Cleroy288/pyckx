//! HTTP API utilities
//!
//! Validation, internal errors, and helper functions for request handling.

pub mod internal;
pub mod validation;

pub use internal::InternalError;
pub use validation::ValidationError;
