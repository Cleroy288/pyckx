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
pub use models::{calculate_cost, DEFAULT_MODEL};
pub use utils::{extract_json_from_response, sanitize_ai_json};
