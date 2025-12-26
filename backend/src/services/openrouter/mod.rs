//! OpenRouter AI Service module
//!
//! Handles communication with OpenRouter API for AI content generation.

mod client;
mod fill_blank;
mod flashcard;
mod keywords;
mod open_question;
mod order_phrase;
mod qcm;
mod true_false;
mod types;
mod utils;
mod verification;

// Course generation module
pub mod simple_course_generation;

// Public API
pub use types::{AnswerGrade, GradedAnswer, OpenRouterService, AVAILABLE_MODELS, DEFAULT_MODEL, validate_model, validate_token_count};



// Utility functions (exposed for tests only)
#[cfg(test)]
pub use utils::{extract_json_from_response, sanitize_json_duplicates};

