//! Flashcard generation and parsing

use super::flashcard_domain::FlashcardAiResponse;
use super::types_domain::{OpenRouterService, Usage};
use super::utils_service::extract_json_from_response;
use crate::services::intello::Flashcard;
use crate::services::intello::QuestionId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::{build_flashcard_prompt, FlashcardPromptInput};
use tracing::{debug, info};

impl OpenRouterService {
    /// Generate flashcards from source content
    pub async fn generate_flashcards(
        &self,
        input: &FlashcardPromptInput,
        model: Option<&str>,
    ) -> Result<(Vec<Flashcard>, Option<Usage>), IntelloError> {
        let prompt = build_flashcard_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending flashcard generation request to OpenRouter"
        );

        let result = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(
            response_length = result.content.len(),
            "Received OpenRouter response"
        );

        let flashcards = self.parse_flashcard_response(&result.content)?;

        info!(
            flashcards_generated = flashcards.len(),
            "Successfully parsed flashcards"
        );

        Ok((flashcards, result.usage))
    }

    /// Parse flashcard response from AI
    fn parse_flashcard_response(&self, response: &str) -> Result<Vec<Flashcard>, IntelloError> {
        let json_str = extract_json_from_response(response);

        let ai_response: FlashcardAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as flashcards: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        let flashcards: Vec<Flashcard> = ai_response
            .cards
            .into_iter()
            .map(|c| Flashcard {
                id: QuestionId::new(),
                front: c.front,
                back: c.back,
            })
            .collect();

        Ok(flashcards)
    }
}
