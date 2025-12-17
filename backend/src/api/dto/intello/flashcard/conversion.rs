//! Flashcard DTO Conversions

use super::response::{FlashcardResponse, FlashcardSetListResponse, FlashcardSetResponse};
use crate::domain::intello::{Flashcard, FlashcardSet};

impl From<&Flashcard> for FlashcardResponse {
    fn from(card: &Flashcard) -> Self {
        Self {
            id: card.id.clone(),
            front: card.front.clone(),
            back: card.back.clone(),
        }
    }
}

impl From<&FlashcardSet> for FlashcardSetResponse {
    fn from(set: &FlashcardSet) -> Self {
        Self {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: format!("{:?}", set.level).to_lowercase(),
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            cards: set.cards.iter().map(FlashcardResponse::from).collect(),
        }
    }
}

impl FlashcardSetListResponse {
    pub fn from_sets(sets: Vec<FlashcardSet>) -> Self {
        let count = sets.len();
        Self {
            sets: sets.iter().map(FlashcardSetResponse::from).collect(),
            count,
        }
    }
}
