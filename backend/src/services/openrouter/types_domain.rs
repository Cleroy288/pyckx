//! OpenRouterService struct, constructor, and DTOs

use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::models_domain::DEFAULT_MODEL;

/// OpenRouter API endpoint
pub(super) const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

/// OpenRouter service for AI-powered game generation
#[derive(Clone, Debug)]
pub struct OpenRouterService {
    pub(super) client: Client,
    pub(super) api_key: String,
    pub(super) model: String,
    /// Optional Google AI API key for higher rate limits with Gemini models
    pub(super) google_ai_key: Option<String>,
}

impl OpenRouterService {
    /// Create a new OpenRouter service
    #[allow(dead_code)]
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: DEFAULT_MODEL.to_string(),
            google_ai_key: None,
        }
    }
    
    /// Create a new OpenRouter service with optional Google AI key
    pub fn with_google_key(api_key: String, google_ai_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: DEFAULT_MODEL.to_string(),
            google_ai_key,
        }
    }
}


// == Request/Response DTOs for OpenRouter API ==

#[derive(Debug, Serialize)]
pub(super) struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
}

#[derive(Debug, Serialize)]
pub(super) struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChatResponse {
    pub choices: Vec<ChatChoice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    #[allow(dead_code)] // Used for comprehensive tracking
    pub total_tokens: u32,
}

/// Usage data from all stages of course generation
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
    pub fn new() -> Self {
        Self::default()
    }
}

/// Result of an AI generation call with usage tracking
#[derive(Debug, Clone)]
pub struct AiGenerationResult {
    pub content: String,
    #[allow(dead_code)] // Used for tracking which model was used
    pub model: String,
    pub usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ChatChoice {
    pub message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
pub(super) struct ResponseMessage {
    pub content: String,
}

// == Grading Types (Public) ==

/// Grade for a user's answer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnswerGrade {
    Right,
    Medium,
    Error,
}

/// A graded answer with feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradedAnswer {
    pub question_id: String,
    pub grade: AnswerGrade,
    pub feedback: String,
}
