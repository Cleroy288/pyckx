//! QCM domain types — questions, sets, requests

use serde::{Deserialize, Serialize};

use super::intello_types::Level;

/// Single QCM question from API
#[derive(Debug, Clone, Deserialize)]
pub struct QcmQuestion {
    pub id: String,
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}

/// QCM set data from API
#[derive(Debug, Clone, Deserialize)]
pub struct QcmSet {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: Level,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<QcmQuestion>,
}

/// Input for creating a QCM question
#[derive(Debug, Clone, Serialize)]
pub struct CreateQcmQuestionInput {
    pub question: String,
    pub wrong_answers: Vec<String>,
    pub right_answer: String,
    pub explanation: String,
}

impl Default for CreateQcmQuestionInput {
    fn default() -> Self {
        Self {
            question: String::new(),
            wrong_answers: vec![
                String::new(),
                String::new(),
                String::new(),
            ],
            right_answer: String::new(),
            explanation: String::new(),
        }
    }
}

/// Request to create a new QCM set
#[derive(Debug, Clone, Serialize)]
pub struct CreateQcmSetRequest {
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subjects: Vec<String>,
    pub questions: Vec<CreateQcmQuestionInput>,
}

/// API response for quick (ephemeral) QCM generation
#[derive(Debug, Clone, Deserialize)]
pub struct QuickQcmApiResponse {
    pub questions: Vec<QcmQuestion>,
    pub level: String,
    pub language: String,
}

impl QuickQcmApiResponse {
    /// Convert to a QcmSet for the player
    pub fn into_qcm_set(self) -> QcmSet {
        let level = Level::parse(&self.level);
        QcmSet {
            id: String::new(),
            user_id: String::new(),
            name: "Quick QCM".into(),
            description: String::new(),
            level,
            language: self.language,
            subjects: Vec::new(),
            questions: self.questions,
        }
    }
}

/// QCM set list response
#[derive(Debug, Clone, Deserialize)]
pub struct QcmSetListResponse {
    pub sets: Vec<QcmSet>,
}

/// QCM success response
#[derive(Debug, Clone, Deserialize)]
pub struct QcmSuccessResponse {
    pub message: String,
    pub set: Option<QcmSet>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_qcm_set() {
        let json = r#"{
            "id": "1", "user_id": "u1",
            "name": "Math", "description": "Desc",
            "level": "easy", "language": "en",
            "subjects": ["algebra"],
            "questions": [{
                "id": "q1",
                "question": "2+2?",
                "wrong_answers": ["3","5","6"],
                "right_answer": "4",
                "explanation": "Basic"
            }]
        }"#;
        let s: QcmSet =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.level, Level::Easy);
        assert_eq!(s.questions.len(), 1);
        assert_eq!(s.questions[0].right_answer, "4");
    }

    #[test]
    fn test_default_create_qcm_question_input() {
        let d = CreateQcmQuestionInput::default();
        assert!(d.question.is_empty());
        assert_eq!(d.wrong_answers.len(), 3);
    }

    #[test]
    fn test_deserialize_qcm_list_response() {
        let json = r#"{"sets": []}"#;
        let r: QcmSetListResponse =
            serde_json::from_str(json).unwrap();
        assert!(r.sets.is_empty());
    }

    #[test]
    fn test_quick_qcm_api_response_into_set() {
        let resp = QuickQcmApiResponse {
            questions: vec![QcmQuestion {
                id: "q1".into(),
                question: "2+2?".into(),
                wrong_answers: vec![
                    "3".into(),
                    "5".into(),
                    "6".into(),
                ],
                right_answer: "4".into(),
                explanation: "Basic".into(),
            }],
            level: "hard".into(),
            language: "fr".into(),
        };
        let set = resp.into_qcm_set();
        assert_eq!(set.level, Level::Hard);
        assert_eq!(set.language, "fr");
        assert_eq!(set.questions.len(), 1);
        assert_eq!(set.name, "Quick QCM");
    }

    #[test]
    fn test_deserialize_qcm_success_null_set() {
        let json =
            r#"{"message":"ok","set":null}"#;
        let r: QcmSuccessResponse =
            serde_json::from_str(json).unwrap();
        assert!(r.set.is_none());
    }
}
