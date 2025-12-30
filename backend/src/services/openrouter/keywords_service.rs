//! Keywords generation and parsing

use super::keywords_domain::KeywordsAiResponse;
use super::types_domain::{OpenRouterService, Usage};
use super::utils_service::extract_json_from_response;
use crate::services::intello::{Keyword, KeywordQuestion};
use crate::services::intello::{OptionId, QuestionId};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::{build_keywords_prompt, KeywordsPromptInput};
use tracing::{debug, info};

impl OpenRouterService {
    /// Generate keywords questions from source content
    pub async fn generate_keywords(
        &self,
        input: &KeywordsPromptInput,
        model: Option<&str>,
    ) -> Result<(Vec<KeywordQuestion>, Option<Usage>), IntelloError> {
        let prompt = build_keywords_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending keywords generation request to OpenRouter"
        );

        let result = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(
            response_length = result.content.len(),
            "Received OpenRouter response"
        );

        let questions = self.parse_keywords_response(&result.content)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed keywords questions"
        );

        Ok((questions, result.usage))
    }

    /// Parse keywords response from AI
    fn parse_keywords_response(
        &self,
        response: &str,
    ) -> Result<Vec<KeywordQuestion>, IntelloError> {
        let json_str = extract_json_from_response(response);

        let ai_response: KeywordsAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as keywords questions: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        let questions: Vec<KeywordQuestion> = ai_response
            .questions
            .into_iter()
            .map(|q| KeywordQuestion {
                id: QuestionId::new(),
                statement: q.statement,
                keywords: q
                    .keywords
                    .into_iter()
                    .map(|k| Keyword {
                        id: OptionId::new(),
                        word: k.word,
                        is_correct: k.is_correct,
                    })
                    .collect(),
                explanation: q.explanation,
            })
            .collect();

        Ok(questions)
    }
}
