//! Keywords AI response types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeywordsAiResponse {
    pub questions: Vec<KeywordsAiQuestion>,
}

#[derive(Debug, Deserialize)]
pub struct KeywordsAiQuestion {
    pub statement: String,
    pub keywords: Vec<KeywordsAiKeyword>,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
pub struct KeywordsAiKeyword {
    pub word: String,
    pub is_correct: bool,
}
