//! Fill Blank generation and parsing

use super::types::OpenRouterService;
use super::utils::extract_json_from_response;
use crate::domain::intello::{FillBlankOption, FillBlankQuestion};
use crate::error::IntelloError;
use crate::shared::{build_fill_blank_prompt, FillBlankPromptInput};
use serde::Deserialize;
use tracing::{debug, info};

// == Fill Blank Response Parser ==

#[derive(Debug, Deserialize)]
struct FillBlankAiResponse {
    questions: Vec<FillBlankAiQuestion>,
}

#[derive(Debug, Deserialize)]
struct FillBlankAiQuestion {
    phrase: String,
    options: Vec<FillBlankAiOption>,
    explanation: String,
}

#[derive(Debug, Deserialize)]
struct FillBlankAiOption {
    text: String,
    is_correct: bool,
}

impl OpenRouterService {
    /// Generate fill blank questions from source content
    pub async fn generate_fill_blank(
        &self,
        input: &FillBlankPromptInput,
        model: Option<&str>,
    ) -> Result<Vec<FillBlankQuestion>, IntelloError> {
        let prompt = build_fill_blank_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending fill blank generation request to OpenRouter"
        );

        let response = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(response_length = response.len(), "Received OpenRouter response");

        let questions = self.parse_fill_blank_response(&response)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed fill blank questions"
        );

        Ok(questions)
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
                id: uuid::Uuid::new_v4().to_string(),
                phrase: q.phrase,
                options: q.options
                    .into_iter()
                    .map(|o| FillBlankOption {
                        id: uuid::Uuid::new_v4().to_string(),
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
