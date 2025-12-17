//! Order Phrase domain entities
//!
//! An OrderPhraseSet is a collection of phrase puzzles where users must
//! arrange shuffled words into the correct order to form a coherent phrase.

use serde::{Deserialize, Serialize};

use super::Level;

/// A single word in a phrase puzzle with its correct position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderPhraseWord {
    /// Unique identifier
    pub id: String,
    /// The word text
    pub word: String,
    /// Correct position in the phrase (0-indexed)
    pub position: u8,
}

/// A single phrase puzzle with words to be ordered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderPhraseQuestion {
    /// Unique identifier
    pub id: String,
    /// The original complete phrase (for display/verification)
    pub original_phrase: String,
    /// The words with their correct positions (sent shuffled to frontend)
    pub words: Vec<OrderPhraseWord>,
    /// Optional hint to help the user
    pub hint: String,
}

impl OrderPhraseQuestion {
    /// Get the words in correct order
    #[allow(dead_code)]
    pub fn words_in_order(&self) -> Vec<&OrderPhraseWord> {
        let mut sorted: Vec<&OrderPhraseWord> = self.words.iter().collect();
        sorted.sort_by_key(|w| w.position);
        sorted
    }

    /// Get the number of words in this puzzle
    #[allow(dead_code)]
    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

/// A set of phrase puzzles for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderPhraseSet {
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
    /// The phrase puzzles in this set
    pub questions: Vec<OrderPhraseQuestion>,
}

impl OrderPhraseSet {
    /// Get the number of questions in this set
    #[allow(dead_code)]
    pub fn question_count(&self) -> usize {
        self.questions.len()
    }
}
