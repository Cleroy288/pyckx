//! Fill Blank Response DTOs

use crate::services::intello::FillBlankOption;
use serde::Serialize;

/// Response for a single option
#[derive(Debug, Serialize)]
pub struct FillBlankOptionResponse {
    pub id: String,
    pub text: String,
    pub is_correct: bool,
}

impl From<&FillBlankOption> for FillBlankOptionResponse {
    fn from(o: &FillBlankOption) -> Self {
        Self {
            id: o.id.to_string(),
            text: o.text.clone(),
            is_correct: o.is_correct,
        }
    }
}

/// Response for a single fill blank question
#[derive(Debug, Serialize)]
pub struct FillBlankQuestionResponse {
    pub id: String,
    pub phrase: String,
    pub options: Vec<FillBlankOptionResponse>,
    pub explanation: String,
}

/// Response for creating fill blank questions (AI generation)
#[derive(Debug, Serialize)]
pub struct CreateFillBlankResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<FillBlankQuestionResponse>,
}

/// Response for listing fill blank sets (with full questions for playing)
#[derive(Debug, Serialize)]
pub struct FillBlankSetListResponse {
    pub sets: Vec<FillBlankSetWithQuestionsResponse>,
    pub count: usize,
}

/// Response for a fill blank set with full questions (for playing)
#[derive(Debug, Serialize)]
pub struct FillBlankSetWithQuestionsResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<FillBlankQuestionResponse>,
}
