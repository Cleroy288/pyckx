//! QCM Request DTOs

use crate::http_api::data_transfer_object::validation::{
    default_language, parse_level, validate_subjects,
};
use crate::services::Level;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal CreateQcmSetRequest
    fn create_req(level: &str) -> CreateQcmSetRequest {
        CreateQcmSetRequest {
            name: "Test".into(),
            description: "Desc".into(),
            level: level.into(),
            language: "en".into(),
            subjects: vec![],
            questions: vec![],
        }
    }

    #[test]
    fn test_create_parse_level_easy() {
        // arrange
        let req = create_req("easy");

        // act
        let result = req.parse_level();

        // assert
        assert_eq!(result.unwrap(), Level::Easy);
    }

    #[test]
    fn test_create_parse_level_invalid() {
        // arrange
        let req = create_req("impossible");

        // act
        let result = req.parse_level();

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_create_validate_subjects_ok() {
        // arrange
        let req = create_req("easy");

        // act
        let result = req.validate_subjects();

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_parse_level_none_returns_ok_none() {
        // arrange
        let req = UpdateQcmSetRequest {
            name: None,
            description: None,
            level: None,
            language: None,
            subjects: None,
            questions: None,
        };

        // act
        let result = req.parse_level();

        // assert
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_update_parse_level_some_valid() {
        // arrange
        let req = UpdateQcmSetRequest {
            name: None,
            description: None,
            level: Some("hard".into()),
            language: None,
            subjects: None,
            questions: None,
        };

        // act
        let result = req.parse_level();

        // assert
        assert_eq!(result.unwrap(), Some(Level::Hard));
    }

    #[test]
    fn test_update_validate_subjects_none_ok() {
        // arrange
        let req = UpdateQcmSetRequest {
            name: None,
            description: None,
            level: None,
            language: None,
            subjects: None,
            questions: None,
        };

        // act
        let result = req.validate_subjects();

        // assert
        assert!(result.is_ok());
    }
}
