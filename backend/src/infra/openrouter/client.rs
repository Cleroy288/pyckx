//! OpenRouter HTTP Client for AI API communication

use reqwest::Client;

use super::models::DEFAULT_MODEL;
use super::types::{
    AiGenerationResult, ChatMessage, ChatRequest, ChatResponse,
    OPENROUTER_API_URL,
};
use crate::services::intello::error_domain::IntelloError;

// ============================================================
// CLIENT STRUCT
// ============================================================

/// HTTP client for OpenRouter API
#[derive(Clone, Debug)]
pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    /// Optional Google AI API key for higher rate limits with Gemini models
    google_ai_key: Option<String>,
}

impl OpenRouterClient {
    /// Create a new OpenRouter client
    #[allow(dead_code)]
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            google_ai_key: None,
        }
    }

    /// Create a new OpenRouter client with optional Google AI key
    pub fn with_google_key(
        api_key: String,
        google_ai_key: Option<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            api_key,
            google_ai_key,
        }
    }

    // ============================================================
    // SEND CHAT REQUEST
    // ============================================================

    /// Send a chat completion request to OpenRouter with optional model override
    pub async fn send_chat_request(
        &self,
        prompt: &str,
        model: Option<&str>,
    ) -> Result<AiGenerationResult, IntelloError> {
        let model_to_use = model.unwrap_or(DEFAULT_MODEL);

        let request = ChatRequest {
            model: model_to_use.to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.7,
        };

        let mut request_builder = self
            .client
            .post(OPENROUTER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "http://localhost:8080")
            .header("X-Title", "Pyckx Educational Games");

        // Add Google AI API key header if available and using a Google/Gemini model
        if let Some(ref google_key) = self.google_ai_key {
            if model_to_use.starts_with("google/") {
                request_builder = request_builder
                    .header("X-Google-AI-Key", google_key.as_str());
            }
        }

        let response =
            request_builder.json(&request).send().await.map_err(|err| {
                IntelloError::external("OpenRouter", err.to_string())
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(IntelloError::external(
                "OpenRouter",
                format!("API error {}: {}", status, error_text),
            ));
        }

        let chat_response: ChatResponse =
            response.json().await.map_err(|err| {
                IntelloError::external(
                    "OpenRouter",
                    format!("Failed to parse response: {}", err),
                )
            })?;

        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| {
                IntelloError::external("OpenRouter", "No response content")
            })?;

        Ok(AiGenerationResult {
            content,
            model: model_to_use.to_string(),
            usage: chat_response.usage,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_client_with_api_key() {
        // arrange / act
        let client = OpenRouterClient::new("test-key".into());

        // assert
        assert_eq!(client.api_key, "test-key");
        assert!(client.google_ai_key.is_none());
    }

    #[test]
    fn test_with_google_key_stores_both_keys() {
        // arrange / act
        let client = OpenRouterClient::with_google_key(
            "api-key".into(),
            Some("google-key".into()),
        );

        // assert
        assert_eq!(client.api_key, "api-key");
        assert_eq!(
            client.google_ai_key,
            Some("google-key".to_string())
        );
    }

    #[test]
    fn test_with_google_key_none_has_no_google_key() {
        // arrange / act
        let client = OpenRouterClient::with_google_key(
            "api-key".into(),
            None,
        );

        // assert
        assert_eq!(client.api_key, "api-key");
        assert!(client.google_ai_key.is_none());
    }
}
