//! Flashcards domain types — re-exported from
//! `crate::domain::flashcard_types` so the rest of the
//! feature has one import surface.

pub use crate::domain::flashcard_types::{
    CreateFlashcardResponse, Flashcard, FlashcardSet,
    FlashcardSetListResponse,
};
