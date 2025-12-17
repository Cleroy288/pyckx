//! Keywords Response DTOs

use crate::domain::intello::Keyword;
use serde::Serialize;

/// Response for a single keyword
#[derive(Debug, Serialize)]
pub struct KeywordResponse {
    pub id: String,
    pub word: String,
    pub is_correct: bool,
}

impl From<&Keyword> for KeywordResponse {
    fn from(k: &Keyword) -> Self {
        Self {
            id: k.id.clone(),
            word: k.word.clone(),
            is_correct: k.is_correct,
        }
    }
}

/// Response for a single keyword question
#[derive(Debug, Serialize)]
pub struct KeywordQuestionResponse {
    pub id: String,
    pub statement: String,
    pub keywords: Vec<KeywordResponse>,
    pub explanation: String,
}

/// Response for creating keyword questions (AI generation)
#[derive(Debug, Serialize)]
pub struct CreateKeywordsResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<KeywordQuestionResponse>,
}

/// Response for listing keyword sets (with full questions for playing)
#[derive(Debug, Serialize)]
pub struct KeywordSetListResponse {
    pub sets: Vec<KeywordSetWithQuestionsResponse>,
    pub count: usize,
}

/// Response for a keyword set with full questions (for playing)
#[derive(Debug, Serialize)]
pub struct KeywordSetWithQuestionsResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<KeywordQuestionResponse>,
}
