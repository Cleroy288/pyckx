//! Keywords AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct KeywordsAiResponse {
    pub questions: Vec<KeywordsAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub(super) struct KeywordsAiQuestion {
    pub statement: String,
    pub keywords: Vec<KeywordsAiKeyword>,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct KeywordsAiKeyword {
    pub word: String,
    pub is_correct: bool,
}
