//! QCM repository types - Row structs for database operations

use crate::services::Level;
use serde::{Deserialize, Serialize};

// =============================================================================
// LEVEL CONVERSION
// =============================================================================

/// Convert Level enum to database string
pub fn level_to_db(level: &Level) -> String {
    match level {
        Level::Easy => "easy".to_string(),
        Level::Medium => "medium".to_string(),
        Level::Hard => "hard".to_string(),
    }
}

/// Convert database string to Level enum
pub fn level_from_db(level: &str) -> Level {
    match level {
        "easy" => Level::Easy,
        "hard" => Level::Hard,
        _ => Level::Medium,
    }
}

// =============================================================================
// QCM SET TYPES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct UpdateQcmSetRow {
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct QcmSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

// =============================================================================
// QCM QUESTION TYPES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct InsertQcmQuestionRow {
    pub id: String,
    pub set_id: String,
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // set_id needed for deserialization
pub struct QcmQuestionRow {
    pub id: String,
    pub set_id: String,
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}
