//! Fill Blank Request DTOs

use crate::http_api::data_transfer_object::validation::{
    default_language, default_num_questions, parse_level,
};
use crate::services::Level;
use serde::Deserialize;

/// Request to create fill blank questions (metadata only, files sent separately)
#[derive(Debug, Deserialize)]
pub struct CreateFillBlankRequest {
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

impl CreateFillBlankRequest {
    pub fn parse_level(&self) -> Result<Level, String> {
        parse_level(&self.level)
    }
}
