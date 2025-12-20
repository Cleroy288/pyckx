//! OpenRouterService struct, constructor, and DTOs

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// OpenRouter API endpoint
pub(super) const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

/// Default model to use
pub const DEFAULT_MODEL: &str = "google/gemini-2.0-flash-exp:free";

/// Safety buffer to subtract from model context limit (10K tokens)
const TOKEN_SAFETY_BUFFER: u32 = 10_000;

/// Model configuration with context limits
#[derive(Debug, Clone)]
pub struct ModelConfig {
    pub id: &'static str,
    pub context_tokens: u32,
}

/// Available AI models with context limits (in tokens)
pub const MODEL_CONFIGS: &[ModelConfig] = &[
    // Free models
    ModelConfig { id: "google/gemini-2.0-flash-exp:free", context_tokens: 1_050_000 },
    ModelConfig { id: "kwaipilot/kat-coder-pro:free", context_tokens: 256_000 },
    ModelConfig { id: "mistralai/devstral-2512:free", context_tokens: 262_000 },
    ModelConfig { id: "tngtech/deepseek-r1t2-chimera:free", context_tokens: 164_000 },
    // Paid models (larger context)
    ModelConfig { id: "google/gemini-3-flash-preview", context_tokens: 1_050_000 },
    ModelConfig { id: "google/gemini-3-pro-preview", context_tokens: 1_050_000 },
    ModelConfig { id: "openai/gpt-5.2", context_tokens: 400_000 },
    ModelConfig { id: "amazon/nova-2-lite-v1", context_tokens: 1_000_000 },
    ModelConfig { id: "x-ai/grok-4.1-fast", context_tokens: 2_000_000 },
];

/// Available AI models for content generation (for backwards compatibility)
pub const AVAILABLE_MODELS: &[&str] = &[
    // Free models
    "google/gemini-2.0-flash-exp:free",
    "kwaipilot/kat-coder-pro:free",
    "mistralai/devstral-2512:free",
    "tngtech/deepseek-r1t2-chimera:free",
    // Paid models
    "google/gemini-3-flash-preview",
    "google/gemini-3-pro-preview",
    "openai/gpt-5.2",
    "amazon/nova-2-lite-v1",
    "x-ai/grok-4.1-fast",
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

/// Get context limit for a model (with safety buffer applied)
pub fn get_model_context_limit(model: &str) -> u32 {
    MODEL_CONFIGS.iter()
        .find(|m| m.id == model)
        .map(|m| m.context_tokens.saturating_sub(TOKEN_SAFETY_BUFFER))
        .unwrap_or(1_040_000)  // Default: 1.05M - 10K safety buffer
}

/// Validate token count against model limit (with safety buffer)
pub fn validate_token_count(model: &str, token_count: u32) -> Result<(), String> {
    let limit = get_model_context_limit(model);
    if token_count > limit {
        Err(format!(
            "Document size ({} tokens) exceeds the model's context limit ({} tokens). Please reduce document size or use fewer/smaller files.",
            token_count, limit
        ))
    } else {
        Ok(())
    }
}

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
