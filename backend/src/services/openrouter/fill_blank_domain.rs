//! Fill Blank AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct FillBlankAiResponse {
    pub questions: Vec<FillBlankAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub(super) struct FillBlankAiQuestion {
    pub phrase: String,
    pub options: Vec<FillBlankAiOption>,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct FillBlankAiOption {
    pub text: String,
    pub is_correct: bool,
}
