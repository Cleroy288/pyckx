//! Fill in the Blank domain entities
//!
//! A FillBlankSet is a collection of fill-in-the-blank questions where users
//! complete phrases by selecting the correct answer from multiple options.

use serde::{Deserialize, Serialize};

use super::Level;

/// A single answer option (may be correct or incorrect)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FillBlankOption {
    /// Unique identifier
    pub id: String,
    /// The option text
    pub text: String,
    /// Whether this option is the correct answer
    pub is_correct: bool,
}

/// A single fill-in-the-blank question with a phrase and answer options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FillBlankQuestion {
    /// Unique identifier
    pub id: String,
    /// The phrase with a blank to fill (e.g., "My name is ___")
    pub phrase: String,
    /// The answer options (exactly one should be correct)
    pub options: Vec<FillBlankOption>,
    /// Explanation of why the correct answer is right
    pub explanation: String,
}

impl FillBlankQuestion {
    /// Get the correct option for this question
    #[allow(dead_code)]
    pub fn correct_option(&self) -> Option<&FillBlankOption> {
        self.options.iter().find(|o| o.is_correct)
    }

    /// Get the number of options
    #[allow(dead_code)]
    pub fn option_count(&self) -> usize {
        self.options.len()
    }
}

/// A set of fill-in-the-blank questions for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FillBlankSet {
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
    /// The fill-in-the-blank questions in this set
    pub questions: Vec<FillBlankQuestion>,
}

impl FillBlankSet {
    /// Get the number of questions in this set
    #[allow(dead_code)]
    pub fn question_count(&self) -> usize {
        self.questions.len()
    }
}
