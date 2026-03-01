//! Flashcard Request DTOs

use crate::http_api::data_transfer_object::intello::validation::{
    default_language, default_num_questions, parse_level,
};
use crate::services::intello::Level;
use serde::Deserialize;

/// Request to create flashcards (metadata only, files sent separately)
#[derive(Debug, Deserialize)]
pub struct CreateFlashcardRequest {
    pub name: String,
    pub description: String,
    pub instructions: String,
    #[serde(default = "default_language")]
    pub language: String,
    pub level: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default = "default_num_questions")]
    pub num_questions: u8,
}

impl CreateFlashcardRequest {
    pub fn parse_level(&self) -> Result<Level, String> {
        parse_level(&self.level)
    }
}
