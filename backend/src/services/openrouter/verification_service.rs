//! Answer verification/grading

use super::verification_domain::VerificationAiResponse;
use super::types_domain::{AnswerGrade, GradedAnswer, OpenRouterService, Usage};
use super::utils_service::extract_json_from_response;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::prompt_builder_service::{build_verification_prompt, VerificationPromptInput};
use tracing::{debug, info};

impl OpenRouterService {
    /// Verify/grade user answers to open questions
    pub async fn verify_answers(
        &self,
        input: &VerificationPromptInput,
    ) -> Result<(Vec<GradedAnswer>, Option<Usage>), IntelloError> {
        let prompt = build_verification_prompt(input);

        info!(
            model = %self.model,
            prompt_length = prompt.len(),
            answers_to_grade = input.answers.len(),
            "Sending answer verification request to OpenRouter"
        );

        let result = self.send_chat_request(&prompt).await?;

        debug!(response_length = result.content.len(), "Received OpenRouter response");

        let grades = self.parse_verification_response(&result.content)?;

        info!(
            grades_returned = grades.len(),
            "Successfully parsed answer grades"
        );

        Ok((grades, result.usage))
    }

    /// Parse verification response from AI
    fn parse_verification_response(&self, response: &str) -> Result<Vec<GradedAnswer>, IntelloError> {
        let json_str = extract_json_from_response(response);

        let ai_response: VerificationAiResponse = serde_json::from_str(&json_str).map_err(|e| {
            IntelloError::validation(
                "ai_response",
                format!(
                    "Failed to parse AI response as verification: {}. Response was: {}",
                    e, json_str
                ),
            )
        })?;

        let grades: Vec<GradedAnswer> = ai_response
            .grades
            .into_iter()
            .map(|g| {
                let grade = match g.grade.to_lowercase().as_str() {
                    "right" => AnswerGrade::Right,
                    "medium" => AnswerGrade::Medium,
                    _ => AnswerGrade::Error,
                };
                GradedAnswer {
                    question_id: g.question_id,
                    grade,
                    feedback: g.feedback,
                }
            })
            .collect();

        Ok(grades)
    }
}
