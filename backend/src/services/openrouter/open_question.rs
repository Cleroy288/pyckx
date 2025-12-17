//! Open question generation and parsing

use super::types::OpenRouterService;
use super::utils::extract_json_from_response;
use crate::domain::intello::OpenQuestion;
use crate::error::IntelloError;
use crate::shared::{build_open_question_prompt, OpenQuestionPromptInput};
use serde::Deserialize;
use tracing::{debug, info};

// == Open Question Response Parser ==

#[derive(Debug, Deserialize)]
struct OpenQuestionAiResponse {
    questions: Vec<OpenQuestionAiQuestion>,
}

#[derive(Debug, Deserialize)]
struct OpenQuestionAiQuestion {
    question: String,
    expected_answer: String,
    hint: String,
}

impl OpenRouterService {
    /// Generate open-ended questions from source content
    pub async fn generate_open_questions(
        &self,
        input: &OpenQuestionPromptInput,
        model: Option<&str>,
    ) -> Result<Vec<OpenQuestion>, IntelloError> {
        let prompt = build_open_question_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending open question generation request to OpenRouter"
        );

        let response = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(response_length = response.len(), "Received OpenRouter response");

        let questions = self.parse_open_question_response(&response)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed open questions"
        );

        Ok(questions)
    }

    /// Parse open question response from AI
    fn parse_open_question_response(
        &self,
        response: &str,
    ) -> Result<Vec<OpenQuestion>, IntelloError> {
        let json_str = extract_json_from_response(response);

        let ai_response: OpenQuestionAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as open questions: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        let questions: Vec<OpenQuestion> = ai_response
            .questions
            .into_iter()
            .map(|q| OpenQuestion {
                id: uuid::Uuid::new_v4().to_string(),
                question: q.question,
                user_answer: String::new(), // Empty initially
                expected_answer: Some(q.expected_answer),
                hint: Some(q.hint),
            })
            .collect();

        Ok(questions)
    }
}
