//! OpenRouter HTTP Client Infrastructure
//!
//! Pure infrastructure layer for communicating with OpenRouter API.
//! Contains ONLY HTTP client logic - no business logic.

mod client;
mod models;
mod types;
mod utils;

// Re-export public API
pub use client::OpenRouterClient;
pub use models::{calculate_cost, get_model, CostResult, ModelDefinition, DEFAULT_MODEL, MODELS};
pub use types::{AiGenerationResult, CourseGenerationUsage, Usage};
pub use utils::{extract_json_from_response, fix_stringified_arrays, sanitize_ai_json, sanitize_json_duplicates};
