//! OpenRouter AI Service module
//!
//! Handles communication with OpenRouter API for AI content generation.
//!
//! # Architecture
//! - Domain entities: `*_domain.rs` files
//! - Business logic: `*_service.rs` files

// Domain modules
mod fill_blank_domain;
mod flashcard_domain;
mod keywords_domain;
pub mod models_domain;
mod open_question_domain;
mod order_phrase_domain;
mod qcm_domain;
mod true_false_domain;
mod types_domain;
mod verification_domain;

// Service modules
mod client_service;
mod fill_blank_service;
mod flashcard_service;
mod keywords_service;
mod open_question_service;
mod order_phrase_service;
mod qcm_service;
mod true_false_service;
pub(crate) mod utils_service;
mod verification_service;

// Course generation modules
pub mod course_generation_prompts;
pub mod course_generation_service;

// Re-export from models_domain (centralized registry)
pub use models_domain::DEFAULT_MODEL;

// Re-export from types_domain
pub use types_domain::{AnswerGrade, GradedAnswer, OpenRouterService};

// Utility functions (exposed for tests only)
#[cfg(test)]
pub use utils_service::{extract_json_from_response, sanitize_json_duplicates};
