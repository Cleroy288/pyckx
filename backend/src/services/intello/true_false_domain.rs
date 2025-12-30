//! True or False domain entities
//!
//! A TrueOrFalseSet is a collection of statements where users must determine
//! if each statement is true or false.

use serde::{Deserialize, Serialize};

use super::enums_domain::Level;
use crate::services::intello::{QuestionId, SetId};
use crate::infra::user::UserId;

/// A single true/false statement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrueOrFalseStatement {
    /// Unique identifier
    pub id: QuestionId,
    /// The statement to judge as true or false
    pub statement: String,
    /// The correct answer (true = statement is true, false = statement is false)
    pub answer: bool,
    /// Explanation of why the statement is true or false
    pub explanation: String,
}

/// A set of true/false statements for a specific topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrueOrFalseSet {
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
    /// The statements in this set
    pub statements: Vec<TrueOrFalseStatement>,
}
