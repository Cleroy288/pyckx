//! Order Phrase generation and parsing

use super::order_phrase_domain::OrderPhraseAiResponse;
use super::types_domain::{OpenRouterService, Usage};
use super::utils_service::extract_json_from_response;
use crate::services::intello::{OrderPhraseQuestion, OrderPhraseWord};
use crate::services::intello::{OptionId, QuestionId};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::{build_order_phrase_prompt, OrderPhrasePromptInput};
use tracing::{debug, info};

impl OpenRouterService {
    /// Generate order phrase questions from source content
    pub async fn generate_order_phrases(
        &self,
        input: &OrderPhrasePromptInput,
        model: Option<&str>,
    ) -> Result<(Vec<OrderPhraseQuestion>, Option<Usage>), IntelloError> {
        let prompt = build_order_phrase_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending order phrase generation request to OpenRouter"
        );

        let result = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(
            response_length = result.content.len(),
            "Received OpenRouter response"
        );

        let questions = self.parse_order_phrase_response(&result.content)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed order phrase questions"
        );

        Ok((questions, result.usage))
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
                id: QuestionId::new(),
                original_phrase: q.original_phrase,
                words: q
                    .words
                    .into_iter()
                    .map(|w| OrderPhraseWord {
                        id: OptionId::new(),
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
