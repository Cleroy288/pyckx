//! QCM (Multiple Choice Quiz) constants
//!
//! Contains constants for QCM quizzes.
//! Domain structs have been moved to domain/intello/.
//! Storage functions have been moved to infrastructure/json_storage/.
//! Validation functions have been moved to services/intello_service.rs.

/// Available games in Intello
#[allow(dead_code)]
pub const AVAILABLE_GAMES: &[&str] = &["qcm", "flashcard"];
