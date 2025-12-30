//! Flashcard domain entities
//!
//! A FlashcardSet is a collection of flashcards for memorization exercises.
//! Each flashcard has a front (question/prompt) and back (answer/explanation).

use serde::{Deserialize, Serialize};

use super::enums_domain::Level;
use crate::services::intello::{QuestionId, SetId};
use crate::infra::user::UserId;

/// A single flashcard for memorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Flashcard {
    /// Unique identifier
    pub id: QuestionId,
    /// Front side of the card (question/term/prompt)
    pub front: String,
    /// Back side of the card (answer/definition/explanation)
    pub back: String,
}

/// A set of flashcards for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlashcardSet {
    /// Unique identifier
    pub id: SetId,
    /// Owner user ID
    pub user_id: UserId,
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
