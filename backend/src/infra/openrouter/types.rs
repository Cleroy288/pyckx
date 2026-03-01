//! HTTP types for OpenRouter API communication

use serde::{Deserialize, Serialize};

/// OpenRouter API endpoint
pub const OPENROUTER_API_URL: &str =
    "https://openrouter.ai/api/v1/chat/completions";

// ============================================================
// REQUEST TYPES
// ============================================================

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
}

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

// ============================================================
// RESPONSE TYPES
// ============================================================

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    #[allow(dead_code)] // Used for comprehensive tracking
    pub total_tokens: u32,
}

// ============================================================
// RESULT TYPE (Returned by client)
// ============================================================

/// Result of an AI generation call with usage tracking
#[derive(Debug, Clone)]
pub struct AiGenerationResult {
    pub content: String,
    #[allow(dead_code)] // Used for tracking which model was used
    pub model: String,
    pub usage: Option<Usage>,
}

// ============================================================
// COURSE GENERATION USAGE (Multi-stage tracking)
// ============================================================

/// Usage data from all stages of course generation
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CourseGenerationUsage {
    /// Stage 0: Demand decryption
    pub decrypt: Option<Usage>,
    /// Stage 0.5: Knowledge expansion (model_id, usage) for each model
    pub expand: Vec<(String, Usage)>,
    /// Stage 1: Knowledge extraction
    pub extract: Option<Usage>,
    /// Stage 1.5: Educational content generation
    pub content: Option<Usage>,
    /// Stage 2: Course generation
    pub generate: Option<Usage>,
    /// JSON repair (if needed)
    pub repair: Option<Usage>,
}

impl CourseGenerationUsage {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}
