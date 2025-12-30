//! Order Phrase domain entities
//!
//! An OrderPhraseSet is a collection of phrase puzzles where users must
//! arrange shuffled words into the correct order to form a coherent phrase.

use serde::{Deserialize, Serialize};

use super::enums_domain::Level;
use crate::services::intello::{OptionId, QuestionId, SetId};
use crate::infra::user::UserId;

/// A single word in a phrase puzzle with its correct position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderPhraseWord {
    /// Unique identifier
    pub id: OptionId,
    /// The word text
    pub word: String,
    /// Correct position in the phrase (0-indexed)
    pub position: u8,
}

/// A single phrase puzzle with words to be ordered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderPhraseQuestion {
    /// Unique identifier
    pub id: QuestionId,
    /// The original complete phrase (for display/verification)
    pub original_phrase: String,
    /// The words with their correct positions (sent shuffled to frontend)
    pub words: Vec<OrderPhraseWord>,
    /// Optional hint to help the user
    pub hint: String,
}

/// A set of phrase puzzles for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderPhraseSet {
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
    /// The phrase puzzles in this set
    pub questions: Vec<OrderPhraseQuestion>,
}
