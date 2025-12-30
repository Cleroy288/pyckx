//! Verification AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct VerificationAiResponse {
    pub grades: Vec<GradedAnswerAi>,
}

#[derive(Debug, Deserialize)]
pub(super) struct GradedAnswerAi {
    pub question_id: String,
    pub grade: String,
    pub feedback: String,
}
