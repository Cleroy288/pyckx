//! Flashcard generation and parsing

use super::types::OpenRouterService;
use super::utils::extract_json_from_response;
use crate::domain::intello::Flashcard;
use crate::error::IntelloError;
use crate::shared::{build_flashcard_prompt, FlashcardPromptInput};
use serde::Deserialize;
use tracing::{debug, info};

// == Flashcard Response Parser ==

#[derive(Debug, Deserialize)]
struct FlashcardAiResponse {
    cards: Vec<FlashcardAiCard>,
}

#[derive(Debug, Deserialize)]
struct FlashcardAiCard {
    front: String,
    back: String,
}

impl OpenRouterService {
    /// Generate flashcards from source content
    pub async fn generate_flashcards(
        &self,
        input: &FlashcardPromptInput,
        model: Option<&str>,
    ) -> Result<Vec<Flashcard>, IntelloError> {
        let prompt = build_flashcard_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending flashcard generation request to OpenRouter"
        );

        let response = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(response_length = response.len(), "Received OpenRouter response");

        let flashcards = self.parse_flashcard_response(&response)?;

        info!(
            flashcards_generated = flashcards.len(),
            "Successfully parsed flashcards"
        );

        Ok(flashcards)
    }

    /// Parse flashcard response from AI
    fn parse_flashcard_response(
        &self,
        response: &str,
    ) -> Result<Vec<Flashcard>, IntelloError> {
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
                id: uuid::Uuid::new_v4().to_string(),
                front: c.front,
                back: c.back,
            })
            .collect();

        Ok(flashcards)
    }
}
