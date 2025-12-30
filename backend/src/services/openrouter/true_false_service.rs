//! True or False generation and parsing

use super::true_false_domain::TrueOrFalseAiResponse;
use super::types_domain::{OpenRouterService, Usage};
use super::utils_service::extract_json_from_response;
use crate::services::intello::TrueOrFalseStatement;
use crate::services::intello::QuestionId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::{build_true_false_prompt, TrueOrFalsePromptInput};
use tracing::{debug, info};

impl OpenRouterService {
    /// Generate true/false statements from source content
    pub async fn generate_true_false(
        &self,
        input: &TrueOrFalsePromptInput,
        model: Option<&str>,
    ) -> Result<(Vec<TrueOrFalseStatement>, Option<Usage>), IntelloError> {
        let prompt = build_true_false_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending true/false generation request to OpenRouter"
        );

        let result = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(
            response_length = result.content.len(),
            "Received OpenRouter response"
        );

        let statements = self.parse_true_false_response(&result.content)?;

        info!(
            statements_generated = statements.len(),
            "Successfully parsed true/false statements"
        );

        Ok((statements, result.usage))
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
                id: QuestionId::new(),
                statement: s.statement,
                answer: s.answer,
                explanation: s.explanation,
            })
            .collect();

        Ok(statements)
    }
}
