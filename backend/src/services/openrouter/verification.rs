//! Answer verification/grading

use super::types::{AnswerGrade, GradedAnswer, OpenRouterService};
use super::utils::extract_json_from_response;
use crate::error::IntelloError;
use crate::shared::{build_verification_prompt, VerificationPromptInput};
use serde::Deserialize;
use tracing::{debug, info};

// == Verification Response Parser ==

#[derive(Debug, Deserialize)]
struct VerificationAiResponse {
    grades: Vec<GradedAnswerAi>,
}

#[derive(Debug, Deserialize)]
struct GradedAnswerAi {
    question_id: String,
    grade: String,
    feedback: String,
}

impl OpenRouterService {
    /// Verify/grade user answers to open questions
    pub async fn verify_answers(
        &self,
        input: &VerificationPromptInput,
    ) -> Result<Vec<GradedAnswer>, IntelloError> {
        let prompt = build_verification_prompt(input);

        info!(
            model = %self.model,
            prompt_length = prompt.len(),
            answers_to_grade = input.answers.len(),
            "Sending answer verification request to OpenRouter"
        );

        let response = self.send_chat_request(&prompt).await?;

        debug!(response_length = response.len(), "Received OpenRouter response");

        let grades = self.parse_verification_response(&response)?;

        info!(
            grades_returned = grades.len(),
            "Successfully parsed answer grades"
        );

        Ok(grades)
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
