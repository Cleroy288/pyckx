//! Open question generation and parsing

use super::open_question_domain::OpenQuestionAiResponse;
use super::types_domain::{OpenRouterService, Usage};
use super::utils_service::{extract_json_from_response, sanitize_json_duplicates};
use crate::services::intello::OpenQuestion;
use crate::services::intello::QuestionId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::{build_open_question_prompt, OpenQuestionPromptInput};
use tracing::{debug, info};

impl OpenRouterService {
    /// Generate open-ended questions from source content
    pub async fn generate_open_questions(
        &self,
        input: &OpenQuestionPromptInput,
        model: Option<&str>,
    ) -> Result<(Vec<OpenQuestion>, Option<Usage>), IntelloError> {
        let prompt = build_open_question_prompt(input);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending open question generation request to OpenRouter"
        );

        let result = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(
            response_length = result.content.len(),
            "Received OpenRouter response"
        );

        let questions = self.parse_open_question_response(&result.content)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed open questions"
        );

        Ok((questions, result.usage))
    }

    /// Parse open question response from AI
    ///
    /// Applies sanitization to remove duplicate JSON keys that AI models
    /// sometimes generate erroneously.
    fn parse_open_question_response(
        &self,
        response: &str,
    ) -> Result<Vec<OpenQuestion>, IntelloError> {
        // Extract JSON from markdown code blocks if present
        let json_str = extract_json_from_response(response);

        // Sanitize duplicate keys (defensive against AI model errors)
        let sanitized_json = sanitize_json_duplicates(&json_str);

        let ai_response: OpenQuestionAiResponse =
            serde_json::from_str(&sanitized_json).map_err(|e| {
                IntelloError::validation(
                    "ai_response",
                    format!(
                        "Failed to parse AI response as open questions: {}. Response was: {}",
                        e, sanitized_json
                    ),
                )
            })?;

        let questions: Vec<OpenQuestion> = ai_response
            .questions
            .into_iter()
            .map(|q| OpenQuestion {
                id: QuestionId::new(),
                question: q.question,
                user_answer: String::new(), // Empty initially
                expected_answer: Some(q.expected_answer),
                hint: Some(q.hint),
            })
            .collect();

        Ok(questions)
    }
}
