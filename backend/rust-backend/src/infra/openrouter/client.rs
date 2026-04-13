//! OpenRouter HTTP Client for AI API communication

use std::time::Duration;

use reqwest::Client;

use super::models::DEFAULT_MODEL;
use super::types::{
    AiGenerationResult, ChatMessage, ChatRequest,
    ChatResponse, OPENROUTER_API_URL,
};
use crate::services::error_domain::StudyError;

/// HTTP-Referer header sent with every request
const HTTP_REFERER: &str = "http://localhost:8080";
/// X-Title header identifying the application
const APP_TITLE: &str = "Pyckx Educational Games";
/// Default temperature for chat completions
const DEFAULT_TEMPERATURE: f32 = 0.7;
/// Maximum response body size (10 MB)
const MAX_RESPONSE_BYTES: usize = 10 * 1024 * 1024;
/// HTTP request timeout in seconds
const REQUEST_TIMEOUT_SECS: u64 = 30;
/// TCP connection timeout in seconds
const CONNECT_TIMEOUT_SECS: u64 = 5;

/// HTTP client for OpenRouter API
#[derive(Clone, Debug)]
pub struct OpenRouterClient {
    client: Client,
    api_key: String,
    /// Google AI API key for Gemini rate limits
    google_ai_key: Option<String>,
}

/// Build a reqwest client with timeouts
fn build_http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .connect_timeout(Duration::from_secs(
            CONNECT_TIMEOUT_SECS,
        ))
        .build()
        .expect("Failed to build HTTP client")
}

impl OpenRouterClient {
    /// Create a new OpenRouter client
    #[allow(dead_code)]
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_http_client(),
            api_key,
            google_ai_key: None,
        }
    }

    /// Create client with optional Google AI key
    pub fn with_google_key(
        api_key: String,
        google_ai_key: Option<String>,
    ) -> Self {
        Self {
            client: build_http_client(),
            api_key,
            google_ai_key,
        }
    }

    /// Send a chat completion request to OpenRouter
    pub async fn send_chat_request(
        &self,
        prompt: &str,
        model: Option<&str>,
    ) -> Result<AiGenerationResult, StudyError> {
        let model_id = model.unwrap_or(DEFAULT_MODEL);
        let builder = self.build_request(prompt, model_id);
        let builder = self.inject_google_header(
            builder, model_id,
        );
        let response = builder.send().await.map_err(|e| {
            StudyError::external("OpenRouter", e.to_string())
        })?;
        Self::parse_response(response, model_id).await
    }
}

/// Request building and response parsing helpers
impl OpenRouterClient {
    /// Build the HTTP request with headers and body
    fn build_request(
        &self,
        prompt: &str,
        model_id: &str,
    ) -> reqwest::RequestBuilder {
        let body = ChatRequest {
            model: model_id.to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: DEFAULT_TEMPERATURE,
        };
        self.client
            .post(OPENROUTER_API_URL)
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key),
            )
            .header("Content-Type", "application/json")
            .header("HTTP-Referer", HTTP_REFERER)
            .header("X-Title", APP_TITLE)
            .json(&body)
    }

    /// Add Google AI key header for Gemini models
    fn inject_google_header(
        &self,
        builder: reqwest::RequestBuilder,
        model_id: &str,
    ) -> reqwest::RequestBuilder {
        match &self.google_ai_key {
            Some(key) if model_id.starts_with("google/") => {
                builder.header("X-Google-AI-Key", key.as_str())
            }
            _ => builder,
        }
    }

    /// Parse and validate the API response
    async fn parse_response(
        response: reqwest::Response,
        model_id: &str,
    ) -> Result<AiGenerationResult, StudyError> {
        if !response.status().is_success() {
            return Err(Self::handle_error(response).await);
        }
        let body = Self::read_bounded_body(response).await?;
        let parsed: ChatResponse =
            serde_json::from_str(&body).map_err(|e| {
                StudyError::external(
                    "OpenRouter",
                    format!("Failed to parse response: {}", e),
                )
            })?;
        let content = parsed
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| {
                StudyError::external(
                    "OpenRouter",
                    "No response content",
                )
            })?;
        Ok(AiGenerationResult {
            content,
            model: model_id.to_string(),
            usage: parsed.usage,
        })
    }
}

/// Error handling and body reading helpers
impl OpenRouterClient {
    /// Build error from non-success response
    async fn handle_error(
        response: reqwest::Response,
    ) -> StudyError {
        let status = response.status();
        let text =
            response.text().await.unwrap_or_default();
        StudyError::external(
            "OpenRouter",
            format!("API error {}: {}", status, text),
        )
    }

    /// Read response body with size limit
    async fn read_bounded_body(
        response: reqwest::Response,
    ) -> Result<String, StudyError> {
        let bytes =
            response.bytes().await.map_err(|e| {
                StudyError::external(
                    "OpenRouter",
                    format!("Failed to read body: {}", e),
                )
            })?;
        if bytes.len() > MAX_RESPONSE_BYTES {
            return Err(StudyError::external(
                "OpenRouter",
                format!(
                    "Response too large: {} bytes",
                    bytes.len()
                ),
            ));
        }
        String::from_utf8(bytes.to_vec()).map_err(|e| {
            StudyError::external(
                "OpenRouter",
                format!("Invalid UTF-8 response: {}", e),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_client_with_api_key() {
        // arrange / act
        let client =
            OpenRouterClient::new("test-key".into());

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
