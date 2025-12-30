//! Open Question Response DTOs

use serde::Serialize;

/// Response for a single open question
#[derive(Debug, Serialize)]
pub struct OpenQuestionResponse {
    pub id: String,
    pub question: String,
    pub user_answer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
}

/// Response for an open question set
#[derive(Debug, Serialize)]
pub struct OpenQuestionSetResponse {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub level: String,
    pub language: String,
    pub subjects: Vec<String>,
    pub questions: Vec<OpenQuestionResponse>,
}

/// Response for list of open question sets
#[derive(Debug, Serialize)]
pub struct OpenQuestionSetListResponse {
    pub sets: Vec<OpenQuestionSetResponse>,
    pub count: usize,
}

/// Response for open question creation
#[derive(Debug, Serialize)]
pub struct CreateOpenQuestionResponse {
    pub success: bool,
    pub message: String,
    pub id: String,
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub questions: Vec<OpenQuestionResponse>,
}

/// Response for a graded answer
#[derive(Debug, Serialize)]
pub struct GradedAnswerResponse {
    pub question_id: String,
    pub grade: String,
    pub feedback: String,
}

/// Response for check answers operation
#[derive(Debug, Serialize)]
pub struct CheckAnswersResponse {
    pub success: bool,
    pub message: String,
    pub set_id: String,
    pub grades: Vec<GradedAnswerResponse>,
}
