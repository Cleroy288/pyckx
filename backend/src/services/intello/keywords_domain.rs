//! Keywords domain entities
//!
//! A KeywordSet is a collection of keyword questions where users must identify
//! which keywords are related to a given statement.

use serde::{Deserialize, Serialize};

use super::enums_domain::Level;
use crate::services::intello::{OptionId, QuestionId, SetId};
use crate::infra::user::UserId;

/// A single keyword option (may be correct or incorrect)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Keyword {
    /// Unique identifier
    pub id: OptionId,
    /// The keyword word/phrase
    pub word: String,
    /// Whether this keyword is correctly related to the statement
    pub is_correct: bool,
}

/// A single keyword question with a statement and keyword options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeywordQuestion {
    /// Unique identifier
    pub id: QuestionId,
    /// The statement/fact to match keywords to
    pub statement: String,
    /// The keyword options (mix of correct and incorrect)
    pub keywords: Vec<Keyword>,
    /// Explanation of why the correct keywords relate to the statement
    pub explanation: String,
}

/// A set of keyword questions for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeywordSet {
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
    /// The keyword questions in this set
    pub questions: Vec<KeywordQuestion>,
}
