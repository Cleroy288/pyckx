//! Flashcard Response DTOs

use serde::Serialize;

/// Response for a single flashcard
#[derive(Debug, Serialize)]
pub struct FlashcardResponse {
    pub id: String,
    pub front: String,
    pub back: String,
}

/// Response for a flashcard set
#[derive(Debug, Serialize)]
pub struct FlashcardSetResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub cards: Vec<FlashcardResponse>,
}

/// Response for list of flashcard sets
#[derive(Debug, Serialize)]
pub struct FlashcardSetListResponse {
    pub sets: Vec<FlashcardSetResponse>,
    pub count: usize,
}

/// Response for flashcard creation
#[derive(Debug, Serialize)]
pub struct CreateFlashcardResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub cards: Vec<FlashcardResponse>,
}
