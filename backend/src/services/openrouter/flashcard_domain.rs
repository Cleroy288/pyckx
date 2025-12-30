//! Flashcard AI response domain types

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct FlashcardAiResponse {
    pub cards: Vec<FlashcardAiCard>,
}

#[derive(Debug, Deserialize)]
pub(super) struct FlashcardAiCard {
    pub front: String,
    pub back: String,
}
