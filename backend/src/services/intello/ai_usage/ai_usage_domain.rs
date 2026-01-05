//! AI Usage Logging - Domain types

use serde::{Deserialize, Serialize};

/// AI usage log entry for tracking costs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiUsageLog {
    pub id: String,
    pub user_id: String,
    pub model_id: String,
    pub feature_type: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub input_cost_usd: f64,
    pub output_cost_usd: f64,
    pub total_cost_usd: f64,
    pub created_at: String,
}

/// Input for creating a usage log entry
#[derive(Debug, Clone)]
pub struct CreateAiUsageLog {
    pub user_id: String,
    pub model_id: String,
    pub feature_type: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub input_cost_usd: f64,
    pub output_cost_usd: f64,
    pub total_cost_usd: f64,
}

/// Feature types for AI usage tracking
#[allow(dead_code)] // Some constants for future course tracking features
pub mod feature_type {
    pub const COURSE_GENERATION: &str = "course_generation";
    pub const KNOWLEDGE_EXPANSION: &str = "knowledge_expansion";
    pub const QCM: &str = "qcm";
    pub const FLASHCARD: &str = "flashcard";
    pub const OPEN_QUESTION: &str = "open_question";
    pub const TRUE_FALSE: &str = "true_false";
    pub const KEYWORDS: &str = "keywords";
    pub const FILL_BLANK: &str = "fill_blank";
    pub const ORDER_PHRASE: &str = "order_phrase";
}
