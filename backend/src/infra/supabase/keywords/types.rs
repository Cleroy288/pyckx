//! Keywords repository types - Row structs for database operations

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
// KEYWORD SET TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct KeywordSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

// =============================================================================
// KEYWORD QUESTION TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct KeywordQuestionRow {
    pub id: String,
    pub set_id: String,
    pub statement: String,
    pub explanation: String,
}

// =============================================================================
// KEYWORD TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct KeywordRow {
    pub id: String,
    pub question_id: String,
    pub word: String,
    pub is_correct: bool,
}
