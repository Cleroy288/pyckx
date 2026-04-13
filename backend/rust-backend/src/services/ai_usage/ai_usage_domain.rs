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

/// Input parameters for logging AI usage
#[derive(Debug, Clone)]
pub struct AiUsageInput<'a> {
    /// The user who initiated the AI request
    pub user_id: &'a str,
    /// The AI model used (e.g., "gpt-4")
    pub model_id: &'a str,
    /// Feature that used AI (e.g., "flashcard", "qcm")
    pub feature_type: &'a str,
    /// Number of input tokens consumed
    pub input_tokens: u32,
    /// Number of output tokens generated
    pub output_tokens: u32,
}

/// Aggregated usage statistics result
#[derive(Debug, Clone)]
pub struct AggregatedUsageStats {
    /// Total number of requests
    pub total_requests: i64,
    /// Total cost in USD
    pub total_cost_usd: f64,
    /// Total input tokens consumed
    pub total_input_tokens: i64,
    /// Total output tokens generated
    pub total_output_tokens: i64,
    /// Stats grouped by feature type
    pub by_feature: Vec<GroupedStats>,
    /// Stats grouped by AI model
    pub by_model: Vec<GroupedStats>,
}

/// Usage statistics for a single grouping key
#[derive(Debug, Clone)]
pub struct GroupedStats {
    /// The grouping key (feature type or model)
    pub key: String,
    /// Number of requests in this group
    pub count: i64,
    /// Total cost for this group
    pub total_cost_usd: f64,
    /// Average cost per request
    pub avg_cost_usd: f64,
    /// Total input tokens for this group
    pub total_input_tokens: i64,
    /// Total output tokens for this group
    pub total_output_tokens: i64,
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
