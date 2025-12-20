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

pub use types::{AnswerGrade, GradedAnswer, OpenRouterService, AVAILABLE_MODELS, DEFAULT_MODEL, validate_model, validate_token_count};



