//! Order Phrase generation and parsing

use super::types::OpenRouterService;
use super::utils::extract_json_from_response;
use crate::domain::intello::{OrderPhraseQuestion, OrderPhraseWord};
use crate::error::IntelloError;
use crate::shared::{build_order_phrase_prompt, OrderPhrasePromptInput};
use serde::Deserialize;
use tracing::{debug, info};

// == Order Phrase Response Parser ==

#[derive(Debug, Deserialize)]
struct OrderPhraseAiResponse {
    questions: Vec<OrderPhraseAiQuestion>,
}

#[derive(Debug, Deserialize)]
struct OrderPhraseAiQuestion {
    original_phrase: String,
    words: Vec<OrderPhraseAiWord>,
    hint: String,
}

#[derive(Debug, Deserialize)]
struct OrderPhraseAiWord {
    word: String,
    position: u8,
}

impl OpenRouterService {
    /// Generate order phrase questions from source content
    pub async fn generate_order_phrases(
        &self,
        input: &OrderPhrasePromptInput,
        model: Option<&str>,
    ) -> Result<Vec<OrderPhraseQuestion>, IntelloError> {
        let prompt = build_order_phrase_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending order phrase generation request to OpenRouter"
        );

        let response = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(response_length = response.len(), "Received OpenRouter response");

        let questions = self.parse_order_phrase_response(&response)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed order phrase questions"
        );

        Ok(questions)
    }

    /// Parse order phrase response from AI
    fn parse_order_phrase_response(
        &self,
        response: &str,
    ) -> Result<Vec<OrderPhraseQuestion>, IntelloError> {
        let json_str = extract_json_from_response(response);

        let ai_response: OrderPhraseAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as order phrase questions: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        let questions: Vec<OrderPhraseQuestion> = ai_response
            .questions
            .into_iter()
            .map(|q| OrderPhraseQuestion {
                id: uuid::Uuid::new_v4().to_string(),
                original_phrase: q.original_phrase,
                words: q.words
                    .into_iter()
                    .map(|w| OrderPhraseWord {
                        id: uuid::Uuid::new_v4().to_string(),
                        word: w.word,
                        position: w.position,
                    })
                    .collect(),
                hint: q.hint,
            })
            .collect();

        Ok(questions)
    }
}
