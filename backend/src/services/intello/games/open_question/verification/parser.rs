//! Verification parser

use crate::infra::openrouter::extract_json_from_response;
use crate::services::intello::ai_response::VerificationAiResponse;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::games::shared::types::{AnswerGrade, GradedAnswer};

/// Parse verification/grading response from AI
pub fn parse_verification_response(response: &str) -> Result<Vec<GradedAnswer>, IntelloError> {
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
        .map(|g| GradedAnswer {
            question_id: g.question_id,
            grade: match g.grade.as_str() {
                "right" => AnswerGrade::Right,
                "medium" => AnswerGrade::Medium,
                _ => AnswerGrade::Error,
            },
            feedback: g.feedback,
        })
        .collect();

    Ok(grades)
}
