//! Keywords Request DTOs

use crate::api::dto::intello::validation::{default_language, default_num_questions, parse_level};
use crate::domain::intello::Level;
use serde::Deserialize;

/// Request to create keyword questions (metadata only, files sent separately)
#[derive(Debug, Deserialize)]
pub struct CreateKeywordsRequest {
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
    /// AI model to use (optional)
    #[serde(default)]
    pub model: Option<String>,
}

impl CreateKeywordsRequest {
    pub fn parse_level(&self) -> Result<Level, String> {
        parse_level(&self.level)
    }
}
