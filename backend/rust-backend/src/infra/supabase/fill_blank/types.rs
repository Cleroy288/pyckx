//! Fill Blank repository types - Row structs for database operations

use crate::services::Level;
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
// FILL BLANK SET TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct FillBlankSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

// =============================================================================
// FILL BLANK QUESTION TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FillBlankQuestionRow {
    pub id: String,
    pub set_id: String,
    pub phrase: String,
    pub explanation: String,
}

// =============================================================================
// FILL BLANK OPTION TYPES
// =============================================================================

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FillBlankOptionRow {
    pub id: String,
    pub question_id: String,
    pub text: String,
    pub is_correct: bool,
}
