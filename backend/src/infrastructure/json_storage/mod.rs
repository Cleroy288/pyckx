//! JSON Storage implementations
//!
//! This module provides repository implementations that use JSON file storage.
//! Legacy/test-only - not used in production (Supabase is used instead).
//!
//! # Structure
//! - `qcm_repository.rs` - JSON-based QCM repository implementation
//! - `open_question_repository.rs` - JSON-based open question repository implementation
//! - `flashcard_repository.rs` - JSON-based flashcard repository implementation

#[allow(dead_code)]
mod flashcard_repository;
#[allow(dead_code)]
mod open_question_repository;
#[allow(dead_code)]
mod qcm_repository;

pub use flashcard_repository::JsonFlashcardRepository;
pub use open_question_repository::JsonOpenQuestionRepository;
pub use qcm_repository::JsonQcmRepository;

