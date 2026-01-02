//! Fill Blank AI response types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FillBlankAiResponse {
    pub questions: Vec<FillBlankAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub struct FillBlankAiQuestion {
    pub phrase: String,
    pub options: Vec<FillBlankAiOption>,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
pub struct FillBlankAiOption {
    pub text: String,
    pub is_correct: bool,
}
