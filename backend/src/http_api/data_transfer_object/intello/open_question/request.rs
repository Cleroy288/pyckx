//! Open Question Request DTOs

use crate::services::intello::Level;
use crate::http_api::data_transfer_object::intello::validation::{
    default_language, default_num_questions, parse_level,
};
use serde::Deserialize;

/// Request to create open questions (metadata only, files sent separately)
#[derive(Debug, Deserialize)]
pub struct CreateOpenQuestionRequest {
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

impl CreateOpenQuestionRequest {
    pub fn parse_level(&self) -> Result<Level, String> {
        parse_level(&self.level)
    }
}

/// Request to check/grade user answers
#[derive(Debug, Deserialize)]
pub struct CheckAnswersRequest {
    pub set_id: String,
    pub answers: Vec<UserAnswerInput>,
}

/// A single user answer to grade
#[derive(Debug, Deserialize)]
pub struct UserAnswerInput {
    pub question_id: String,
    pub user_answer: String,
}
