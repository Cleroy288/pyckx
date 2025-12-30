//! True or False repository types - Row structs for database operations

use crate::services::intello::Level;
use serde::Deserialize;

// =============================================================================
// LEVEL CONVERSION
// =============================================================================

pub fn level_to_db(level: &Level) -> String {
    match level {
        Level::Easy => "easy".to_string(),
        Level::Medium => "medium".to_string(),
        Level::Hard => "hard".to_string(),
    }
}

pub fn level_from_db(level: &str) -> Level {
    match level {
        "easy" => Level::Easy,
        "hard" => Level::Hard,
        _ => Level::Medium,
    }
}

// =============================================================================
// TRUE OR FALSE SET TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct TrueOrFalseSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

// =============================================================================
// TRUE OR FALSE STATEMENT TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // set_id needed for deserialization
pub struct TrueOrFalseStatementRow {
    pub id: String,
    pub set_id: String,
    pub statement: String,
    pub answer: bool,
    pub explanation: String,
}
