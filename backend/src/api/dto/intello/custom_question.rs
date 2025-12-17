//! Custom Question DTOs

use crate::domain::intello::Level;
use crate::api::dto::intello::validation::{
    validate_subjects, parse_level, validate_num_questions, default_language, default_num_questions
};
use crate::api::dto::intello::qcm::QcmQuestionResponse;
use serde::{Deserialize, Serialize};

/// Request to create a custom question configuration (metadata only, files sent separately)
#[derive(Debug, Deserialize)]
pub struct CreateCustomQuestionRequest {
    pub name: String,
    pub description: String,
    pub instructions: String,
    #[serde(default = "default_language")]
    pub language: String,
    pub level: String,
    #[allow(dead_code)] // Reserved for future multi-game support
    pub output_game: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default = "default_num_questions")]
    pub num_questions: u8,
    /// AI model to use (optional, defaults to amazon/nova)
    #[serde(default)]
    pub model: Option<String>,
}

impl CreateCustomQuestionRequest {
    pub fn parse_level(&self) -> Result<Level, String> {
        parse_level(&self.level)
    }

    pub fn validate_subjects(&self) -> Result<(), String> {
        validate_subjects(&self.subjects)
    }

    pub fn validate_num_questions(&self) -> Result<(), String> {
        validate_num_questions(self.num_questions)
    }
}

/// Response for custom question creation
#[derive(Debug, Serialize)]
pub struct CustomQuestionResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    /// Generated questions (for QCM game)
    pub questions: Vec<QcmQuestionResponse>,
}
