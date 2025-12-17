//! Keywords generation and parsing

use super::types::OpenRouterService;
use super::utils::extract_json_from_response;
use crate::domain::intello::{Keyword, KeywordQuestion};
use crate::error::IntelloError;
use crate::shared::{build_keywords_prompt, KeywordsPromptInput};
use serde::Deserialize;
use tracing::{debug, info};

// == Keywords Response Parser ==

#[derive(Debug, Deserialize)]
struct KeywordsAiResponse {
    questions: Vec<KeywordsAiQuestion>,
}

#[derive(Debug, Deserialize)]
struct KeywordsAiQuestion {
    statement: String,
    keywords: Vec<KeywordsAiKeyword>,
    explanation: String,
}

#[derive(Debug, Deserialize)]
struct KeywordsAiKeyword {
    word: String,
    is_correct: bool,
}

impl OpenRouterService {
    /// Generate keywords questions from source content
    pub async fn generate_keywords(
        &self,
        input: &KeywordsPromptInput,
        model: Option<&str>,
    ) -> Result<Vec<KeywordQuestion>, IntelloError> {
        let prompt = build_keywords_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending keywords generation request to OpenRouter"
        );

        let response = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(response_length = response.len(), "Received OpenRouter response");

        let questions = self.parse_keywords_response(&response)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed keywords questions"
        );

        Ok(questions)
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
                id: uuid::Uuid::new_v4().to_string(),
                statement: q.statement,
                keywords: q.keywords
                    .into_iter()
                    .map(|k| Keyword {
                        id: uuid::Uuid::new_v4().to_string(),
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
