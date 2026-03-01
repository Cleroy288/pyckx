//! Quick QCM DTOs — lightweight ephemeral generation

use crate::http_api::data_transfer_object::intello::qcm::QcmQuestionResponse;
use crate::http_api::data_transfer_object::intello::validation::{
    default_language, default_num_questions, parse_level,
};
use crate::services::intello::Level;
use serde::{Deserialize, Serialize};

/// Request metadata for quick (ephemeral) QCM generation
#[derive(Debug, Deserialize)]
pub struct QuickQcmRequest {
    #[serde(default = "default_level")]
    pub level: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_num_questions")]
    pub num_questions: u8,
}

impl QuickQcmRequest {
    /// Parse level string into domain Level
    pub fn parse_level(&self) -> Result<Level, String> {
        parse_level(&self.level)
    }
}

/// Response for ephemeral QCM generation
#[derive(Debug, Serialize)]
pub struct QuickQcmResponse {
    pub questions: Vec<QcmQuestionResponse>,
    pub level: String,
    pub language: String,
}

impl From<&crate::services::intello::QcmSet>
    for QuickQcmResponse
{
    fn from(
        set: &crate::services::intello::QcmSet,
    ) -> Self {
        let level = match set.level {
            Level::Easy => "easy",
            Level::Medium => "medium",
            Level::Hard => "hard",
        };
        Self {
            questions: set
                .questions
                .iter()
                .map(QcmQuestionResponse::from)
                .collect(),
            level: level.to_string(),
            language: set.language.clone(),
        }
    }
}

/// Default level for quick QCM
fn default_level() -> String {
    "medium".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_qcm_request_parse_level_valid() {
        // arrange
        let req = QuickQcmRequest {
            level: "hard".into(),
            language: "en".into(),
            num_questions: 10,
        };

        // act
        let result = req.parse_level();

        // assert
        assert_eq!(result.unwrap(), Level::Hard);
    }

    #[test]
    fn test_quick_qcm_request_parse_level_invalid() {
        // arrange
        let req = QuickQcmRequest {
            level: "expert".into(),
            language: "en".into(),
            num_questions: 10,
        };

        // act
        let result = req.parse_level();

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_default_level_is_medium() {
        assert_eq!(default_level(), "medium");
    }

    #[test]
    fn test_quick_qcm_request_deserialize_defaults() {
        // arrange
        let json = "{}";

        // act
        let req: QuickQcmRequest =
            serde_json::from_str(json).unwrap();

        // assert
        assert_eq!(req.level, "medium");
        assert_eq!(req.language, "en");
        assert_eq!(req.num_questions, 10);
    }

    #[test]
    fn test_quick_qcm_response_from_set_level() {
        use crate::infra::user::UserId;
        use crate::services::intello::{
            QcmQuestion, QcmSet, QuestionId, SetId,
        };

        // arrange
        let set = QcmSet {
            id: SetId::new(),
            user_id: UserId::from("u1"),
            name: "Quick".into(),
            description: String::new(),
            level: Level::Hard,
            language: "fr".into(),
            subjects: Vec::new(),
            questions: vec![QcmQuestion {
                id: QuestionId::new(),
                question: "Q?".into(),
                wrong_answers: vec![
                    "a".into(),
                    "b".into(),
                    "c".into(),
                ],
                right_answer: "d".into(),
                explanation: "E".into(),
            }],
        };

        // act
        let resp = QuickQcmResponse::from(&set);

        // assert
        assert_eq!(resp.level, "hard");
        assert_eq!(resp.language, "fr");
        assert_eq!(resp.questions.len(), 1);
    }
}
