//! Open question domain types

use serde::{Deserialize, Serialize};

/// Single open question
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OpenQuestion {
    pub id: String,
    pub question: String,
    pub user_answer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// Open question set
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OpenQuestionSet {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<OpenQuestion>,
}

/// List response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OpenQuestionSetListResponse {
    pub sets: Vec<OpenQuestionSet>,
    pub count: usize,
}

/// Graded answer from AI
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GradedAnswer {
    pub question_id: String,
    pub grade: String,
    pub feedback: String,
}

/// Check answers response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CheckAnswersResponse {
    pub success: bool,
    pub message: String,
    pub set_id: String,
    pub grades: Vec<GradedAnswer>,
}

/// User answer input for grading
#[derive(Debug, Clone, Serialize)]
pub struct UserAnswerInput {
    pub question_id: String,
    pub user_answer: String,
}

/// Check answers request
#[derive(Debug, Clone, Serialize)]
pub struct CheckAnswersRequest {
    pub set_id: String,
    pub answers: Vec<UserAnswerInput>,
}

/// AI creation response
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CreateOpenQuestionResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<OpenQuestion>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_open_question_set() {
        let json = r#"{
            "id": "1", "user_id": "u",
            "name": "Q", "description": "D",
            "level": "easy", "language": "en",
            "subjects": [],
            "questions": [{
                "id": "q1",
                "question": "What is 1+1?",
                "user_answer": "",
                "expected_answer": "2",
                "hint": "simple"
            }]
        }"#;
        let s: OpenQuestionSet =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.questions.len(), 1);
    }

    #[test]
    fn test_deserialize_graded_answer() {
        let json = r#"{
            "question_id": "q1",
            "grade": "A", "feedback": "Great"
        }"#;
        let g: GradedAnswer =
            serde_json::from_str(json).unwrap();
        assert_eq!(g.grade, "A");
    }

    #[test]
    fn test_serialize_check_answers_request() {
        let req = CheckAnswersRequest {
            set_id: "s1".into(),
            answers: vec![UserAnswerInput {
                question_id: "q1".into(),
                user_answer: "2".into(),
            }],
        };
        let json =
            serde_json::to_string(&req).unwrap();
        assert!(json.contains("q1"));
    }
}
