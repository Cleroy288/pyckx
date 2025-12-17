//! Order Phrase Response DTOs

use crate::domain::intello::OrderPhraseWord;
use serde::Serialize;

/// Response for a single word in a phrase puzzle
#[derive(Debug, Serialize)]
pub struct OrderPhraseWordResponse {
    pub id: String,
    pub word: String,
    pub position: u8,
}

impl From<&OrderPhraseWord> for OrderPhraseWordResponse {
    fn from(w: &OrderPhraseWord) -> Self {
        Self {
            id: w.id.clone(),
            word: w.word.clone(),
            position: w.position,
        }
    }
}

/// Response for a single order phrase question
#[derive(Debug, Serialize)]
pub struct OrderPhraseQuestionResponse {
    pub id: String,
    pub original_phrase: String,
    pub words: Vec<OrderPhraseWordResponse>,
    pub hint: String,
}

/// Response for creating order phrase questions (AI generation)
#[derive(Debug, Serialize)]
pub struct CreateOrderPhraseResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<OrderPhraseQuestionResponse>,
}

/// Response for listing order phrase sets (with full questions for playing)
#[derive(Debug, Serialize)]
pub struct OrderPhraseSetListResponse {
    pub sets: Vec<OrderPhraseSetWithQuestionsResponse>,
    pub count: usize,
}

/// Response for an order phrase set with full questions (for playing)
#[derive(Debug, Serialize)]
pub struct OrderPhraseSetWithQuestionsResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<OrderPhraseQuestionResponse>,
}
