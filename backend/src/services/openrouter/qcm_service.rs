//! QCM generation and parsing

use super::qcm_domain::QcmAiResponse;
use super::types_domain::{OpenRouterService, Usage};
use super::utils_service::extract_json_from_response;
use crate::services::intello::{CustomQuestion, QcmQuestion};
use crate::services::intello::QuestionId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::build_prompt;
use tracing::{debug, info};

impl OpenRouterService {
    /// Generate QCM questions from a CustomQuestion
    pub async fn generate_qcm(
        &self,
        custom_question: &CustomQuestion,
        model: Option<&str>,
    ) -> Result<(Vec<QcmQuestion>, Option<Usage>), IntelloError> {
        let prompt = build_prompt(custom_question);

        let model_display = model.unwrap_or("default");
        info!(
            model = %model_display,
            prompt_length = prompt.len(),
            "Sending QCM generation request to OpenRouter"
        );

        let result = self.send_chat_request_with_model(&prompt, model).await?;

        debug!(
            response_length = result.content.len(),
            "Received OpenRouter response"
        );

        let questions = self.parse_qcm_response(&result.content)?;

        info!(
            questions_generated = questions.len(),
            "Successfully parsed QCM questions"
        );

        Ok((questions, result.usage))
    }

    /// Parse QCM response from AI
    fn parse_qcm_response(&self, response: &str) -> Result<Vec<QcmQuestion>, IntelloError> {
        // Try to extract JSON from the response (AI might include markdown code blocks)
        let json_str = extract_json_from_response(response);

        let ai_response: QcmAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as QCM: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        // Convert to domain QcmQuestion
        let questions: Vec<QcmQuestion> = ai_response
            .questions
            .into_iter()
            .map(|q| QcmQuestion {
                id: QuestionId::new(),
                question: q.question,
                wrong_answers: q.wrong_answers,
                right_answer: q.right_answer,
                explanation: q.explanation,
            })
            .collect();

        Ok(questions)
    }
}
