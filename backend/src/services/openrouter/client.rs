//! HTTP client for OpenRouter API

use super::types::{ChatMessage, ChatRequest, ChatResponse, OpenRouterService, OPENROUTER_API_URL, DEFAULT_MODEL};
use crate::error::IntelloError;

impl OpenRouterService {
    /// Send a chat completion request to OpenRouter with optional model override
    pub(super) async fn send_chat_request_with_model(
        &self,
        prompt: &str,
        model: Option<&str>,
    ) -> Result<String, IntelloError> {
        let model_to_use = model.unwrap_or(DEFAULT_MODEL);
        
        let request = ChatRequest {
            model: model_to_use.to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.7,
        };

        let response = self
            .client
            .post(OPENROUTER_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", "http://localhost:8080")
            .header("X-Title", "Pyckx Educational Games")
            .json(&request)
            .send()
            .await
            .map_err(|e| IntelloError::external("OpenRouter", e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(IntelloError::external(
                "OpenRouter",
                format!("API error {}: {}", status, error_text),
            ));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| IntelloError::external("OpenRouter", format!("Failed to parse response: {}", e)))?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| IntelloError::external("OpenRouter", "No response content"))
    }

    /// Send a chat completion request to OpenRouter (uses default model)
    pub(super) async fn send_chat_request(&self, prompt: &str) -> Result<String, IntelloError> {
        self.send_chat_request_with_model(prompt, None).await
    }
}
