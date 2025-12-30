//! Fill Blank generation and parsing

use super::fill_blank_domain::FillBlankAiResponse;
use super::types_domain::{OpenRouterService, Usage};
use super::utils_service::extract_json_from_response;
use crate::services::intello::{FillBlankOption, FillBlankQuestion};
use crate::services::intello::{OptionId, QuestionId};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::{build_fill_blank_prompt, FillBlankPromptInput};
use tracing::{debug, info};

impl OpenRouterService {
    /// Generate fill blank questions from source content
    pub async fn generate_fill_blank(
        &self,
        input: &FillBlankPromptInput,
        model: Option<&str>,
    ) -> Result<(Vec<FillBlankQuestion>, Option<Usage>), IntelloError> {
        let prompt = build_fill_blank_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending fill blank generation request to OpenRouter"
        );

        let result = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(
            response_length = result.content.len(),
            "Received OpenRouter response"
        );

        let questions = self.parse_fill_blank_response(&result.content)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed fill blank questions"
        );

        Ok((questions, result.usage))
    }

    /// Parse fill blank response from AI
    fn parse_fill_blank_response(
        &self,
        response: &str,
    ) -> Result<Vec<FillBlankQuestion>, IntelloError> {
        let json_str = extract_json_from_response(response);

        let ai_response: FillBlankAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as fill blank questions: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        let questions: Vec<FillBlankQuestion> = ai_response
            .questions
            .into_iter()
            .map(|q| FillBlankQuestion {
                id: QuestionId::new(),
                phrase: q.phrase,
                options: q
                    .options
                    .into_iter()
                    .map(|o| FillBlankOption {
                        id: OptionId::new(),
                        text: o.text,
                        is_correct: o.is_correct,
                    })
                    .collect(),
                explanation: q.explanation,
            })
            .collect();

        Ok(questions)
    }
}
