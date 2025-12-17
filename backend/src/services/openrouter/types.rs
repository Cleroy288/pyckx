//! OpenRouterService struct, constructor, and DTOs

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// OpenRouter API endpoint
pub(super) const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

/// Default model to use
pub const DEFAULT_MODEL: &str = "amazon/nova-2-lite-v1:free";

/// Available AI models for content generation
pub const AVAILABLE_MODELS: &[&str] = &[
    "nvidia/nemotron-nano-12b-v2-vl:free",
    "tngtech/deepseek-r1t-chimera:free",
    "z-ai/glm-4.5-air:free",
    "nex-agi/deepseek-v3.1-nex-n1:free",
    "amazon/nova-2-lite-v1:free",
    "tngtech/deepseek-r1t2-chimera:free",
];

/// Validate that a model is in the available models list
pub fn validate_model(model: &str) -> Result<(), String> {
    if AVAILABLE_MODELS.contains(&model) {
        Ok(())
    } else {
        Err(format!(
            "Invalid model '{}'. Available: {:?}",
            model, AVAILABLE_MODELS
        ))
    }
}

/// OpenRouter service for AI-powered game generation
#[derive(Clone, Debug)]
pub struct OpenRouterService {
    pub(super) client: Client,
    pub(super) api_key: String,
    pub(super) model: String,
}

impl OpenRouterService {
    /// Create a new OpenRouter service
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: DEFAULT_MODEL.to_string(),
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
