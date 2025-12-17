//! Keywords domain entities
//!
//! A KeywordSet is a collection of keyword questions where users must identify
//! which keywords are related to a given statement.

use serde::{Deserialize, Serialize};

use super::Level;

/// A single keyword option (may be correct or incorrect)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keyword {
    /// Unique identifier
    pub id: String,
    /// The keyword word/phrase
    pub word: String,
    /// Whether this keyword is correctly related to the statement
    pub is_correct: bool,
}

/// A single keyword question with a statement and keyword options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeywordQuestion {
    /// Unique identifier
    pub id: String,
    /// The statement/fact to match keywords to
    pub statement: String,
    /// The keyword options (mix of correct and incorrect)
    pub keywords: Vec<Keyword>,
    /// Explanation of why the correct keywords relate to the statement
    pub explanation: String,
}

impl KeywordQuestion {
    /// Get the number of correct keywords
    #[allow(dead_code)]
    pub fn correct_count(&self) -> usize {
        self.keywords.iter().filter(|k| k.is_correct).count()
    }

    /// Get the number of incorrect keywords
    #[allow(dead_code)]
    pub fn incorrect_count(&self) -> usize {
        self.keywords.iter().filter(|k| !k.is_correct).count()
    }
}

/// A set of keyword questions for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeywordSet {
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
    /// The keyword questions in this set
    pub questions: Vec<KeywordQuestion>,
}

impl KeywordSet {
    /// Get the number of questions in this set
    #[allow(dead_code)]
    pub fn question_count(&self) -> usize {
        self.questions.len()
    }
}
