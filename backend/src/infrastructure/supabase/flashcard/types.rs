//! Flashcard repository types - Row structs for database operations

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
// FLASHCARD SET TYPES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct InsertFlashcardSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct FlashcardSetRow {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
}

// =============================================================================
// FLASHCARD TYPES
// =============================================================================

#[derive(Debug, Serialize)]
pub struct InsertFlashcardRow {
    pub id: String,
    pub set_id: String,
    pub front: String,
    pub back: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // set_id needed for deserialization
pub struct FlashcardRow {
    pub id: String,
    pub set_id: String,
    pub front: String,
    pub back: String,
}
