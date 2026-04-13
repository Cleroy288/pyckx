//! Flashcard AI response types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FlashcardAiResponse {
    pub cards: Vec<FlashcardAiCard>,
}

#[derive(Debug, Deserialize)]
pub struct FlashcardAiCard {
    pub front: String,
    pub back: String,
}
