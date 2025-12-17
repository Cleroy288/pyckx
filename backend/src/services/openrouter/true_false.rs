//! True or False generation and parsing

use super::types::OpenRouterService;
use super::utils::extract_json_from_response;
use crate::domain::intello::TrueOrFalseStatement;
use crate::error::IntelloError;
use crate::shared::{build_true_false_prompt, TrueOrFalsePromptInput};
use serde::Deserialize;
use tracing::{debug, info};

// == True or False Response Parser ==

#[derive(Debug, Deserialize)]
struct TrueOrFalseAiResponse {
    statements: Vec<TrueOrFalseAiStatement>,
}

#[derive(Debug, Deserialize)]
struct TrueOrFalseAiStatement {
    statement: String,
    answer: bool,
    explanation: String,
}

impl OpenRouterService {
    /// Generate true/false statements from source content
    pub async fn generate_true_false(
        &self,
        input: &TrueOrFalsePromptInput,
        model: Option<&str>,
    ) -> Result<Vec<TrueOrFalseStatement>, IntelloError> {
        let prompt = build_true_false_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending true/false generation request to OpenRouter"
        );

        let response = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(response_length = response.len(), "Received OpenRouter response");

        let statements = self.parse_true_false_response(&response)?;

        info!(
            statements_generated = statements.len(),
            "Successfully parsed true/false statements"
        );

        Ok(statements)
    }

    /// Parse true/false response from AI
    fn parse_true_false_response(
        &self,
        response: &str,
    ) -> Result<Vec<TrueOrFalseStatement>, IntelloError> {
        let json_str = extract_json_from_response(response);

        let ai_response: TrueOrFalseAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as true/false statements: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        let statements: Vec<TrueOrFalseStatement> = ai_response
            .statements
            .into_iter()
            .map(|s| TrueOrFalseStatement {
                id: uuid::Uuid::new_v4().to_string(),
                statement: s.statement,
                answer: s.answer,
                explanation: s.explanation,
            })
            .collect();

        Ok(statements)
    }
}
