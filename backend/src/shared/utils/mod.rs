//! Shared utilities
//!
//! Generic utility functions with no feature-specific dependencies.

pub mod string_or_vec_deserializer;

pub use string_or_vec_deserializer::deserialize_string_or_vec;
