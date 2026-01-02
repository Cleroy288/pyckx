//! Verification AI response types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VerificationAiResponse {
    pub grades: Vec<GradedAnswerAi>,
}

#[derive(Debug, Deserialize)]
pub struct GradedAnswerAi {
    pub question_id: String,
    pub grade: String,
    pub feedback: String,
}
