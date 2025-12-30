//! Fill in the Blank domain entities
//!
//! A FillBlankSet is a collection of fill-in-the-blank questions where users
//! complete phrases by selecting the correct answer from multiple options.

use serde::{Deserialize, Serialize};

use super::enums_domain::Level;
use crate::services::intello::{OptionId, QuestionId, SetId};
use crate::infra::user::UserId;

/// A single answer option (may be correct or incorrect)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FillBlankOption {
    /// Unique identifier
    pub id: OptionId,
    /// The option text
    pub text: String,
    /// Whether this option is the correct answer
    pub is_correct: bool,
}

/// A single fill-in-the-blank question with a phrase and answer options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FillBlankQuestion {
    /// Unique identifier
    pub id: QuestionId,
    /// The phrase with a blank to fill (e.g., "My name is ___")
    pub phrase: String,
    /// The answer options (exactly one should be correct)
    pub options: Vec<FillBlankOption>,
    /// Explanation of why the correct answer is right
    pub explanation: String,
}

/// A set of fill-in-the-blank questions for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FillBlankSet {
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
    /// The fill-in-the-blank questions in this set
    pub questions: Vec<FillBlankQuestion>,
}
