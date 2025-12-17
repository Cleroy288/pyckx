//! Flashcard domain entities
//!
//! A FlashcardSet is a collection of flashcards for memorization exercises.
//! Each flashcard has a front (question/prompt) and back (answer/explanation).

use serde::{Deserialize, Serialize};

use super::Level;

/// A single flashcard for memorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Flashcard {
    /// Unique identifier
    pub id: String,
    /// Front side of the card (question/term/prompt)
    pub front: String,
    /// Back side of the card (answer/definition/explanation)
    pub back: String,
}

/// A set of flashcards for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlashcardSet {
    /// Unique identifier
    pub id: String,
    /// Owner user ID
    pub user_id: String,
    /// Set name/title
    pub name: String,
    /// Set description
    pub description: String,
    /// Difficulty level
    pub level: Level,
    /// Language of the content
    pub language: String,
    /// Subject tags (max 3)
    pub subjects: Vec<String>,
    /// The flashcards in this set
    pub cards: Vec<Flashcard>,
}

impl FlashcardSet {
    /// Get the number of cards in this set
    #[allow(dead_code)] // Available for future use
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }
}
