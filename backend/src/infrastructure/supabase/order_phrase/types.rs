//! Order Phrase repository types - Row structs for database operations

use crate::domain::intello::Level;
use serde::{Deserialize, Serialize};

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
// ORDER PHRASE SET TYPES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct InsertOrderPhraseSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct OrderPhraseSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

// =============================================================================
// ORDER PHRASE QUESTION TYPES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct InsertOrderPhraseQuestionRow {
    pub id: String,
    pub set_id: String,
    pub original_phrase: String,
    pub hint: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OrderPhraseQuestionRow {
    pub id: String,
    pub set_id: String,
    pub original_phrase: String,
    pub hint: String,
}

// =============================================================================
// ORDER PHRASE WORD TYPES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct InsertOrderPhraseWordRow {
    pub id: String,
    pub question_id: String,
    pub word: String,
    pub position: u8,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OrderPhraseWordRow {
    pub id: String,
    pub question_id: String,
    pub word: String,
    pub position: i32,
}
