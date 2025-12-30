//! QCM Request DTOs

use crate::services::intello::Level;
use crate::http_api::data_transfer_object::intello::validation::{
    default_language, parse_level, validate_subjects,
};
use serde::Deserialize;

/// Request to create a new QCM set
#[derive(Debug, Deserialize)]
pub struct CreateQcmSetRequest {
    pub name: String,
    pub description: String,
    pub level: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub subjects: Vec<String>,
    #[serde(default)]
    pub questions: Vec<QcmQuestionRequest>,
}

/// Request for a single QCM question
#[derive(Debug, Deserialize)]
pub struct QcmQuestionRequest {
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}

/// Request to update a QCM set
#[derive(Debug, Deserialize)]
pub struct UpdateQcmSetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub level: Option<String>,
    pub language: Option<String>,
    pub subjects: Option<Vec<String>>,
    pub questions: Option<Vec<QcmQuestionRequest>>,
}

impl CreateQcmSetRequest {
    pub fn parse_level(&self) -> Result<Level, String> {
        parse_level(&self.level)
    }

    pub fn validate_subjects(&self) -> Result<(), String> {
        validate_subjects(&self.subjects)
    }
}

impl UpdateQcmSetRequest {
    pub fn parse_level(&self) -> Result<Option<Level>, String> {
        match &self.level {
            Some(l) => parse_level(l).map(Some),
            None => Ok(None),
        }
    }

    pub fn validate_subjects(&self) -> Result<(), String> {
        match &self.subjects {
            Some(subjects) => validate_subjects(subjects),
            None => Ok(()),
        }
    }
}
