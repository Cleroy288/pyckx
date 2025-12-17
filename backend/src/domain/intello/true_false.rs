//! True or False domain entities
//!
//! A TrueOrFalseSet is a collection of statements where users must determine
//! if each statement is true or false.

use serde::{Deserialize, Serialize};

use super::Level;

/// A single true/false statement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrueOrFalseStatement {
    /// Unique identifier
    pub id: String,
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
    /// The statements in this set
    pub statements: Vec<TrueOrFalseStatement>,
}

impl TrueOrFalseSet {
    /// Get the number of statements in this set
    #[allow(dead_code)]
    pub fn statement_count(&self) -> usize {
        self.statements.len()
    }
}
