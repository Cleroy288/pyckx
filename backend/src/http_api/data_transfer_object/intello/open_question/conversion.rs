//! Open Question DTO Conversions

use super::response::{
    GradedAnswerResponse, OpenQuestionResponse, OpenQuestionSetListResponse,
    OpenQuestionSetResponse,
};
use crate::services::intello::{Level, OpenQuestion, OpenQuestionSet};
use crate::services::intello::ai_parsing_service::{AnswerGrade, GradedAnswer};

impl From<&OpenQuestion> for OpenQuestionResponse {
    fn from(q: &OpenQuestion) -> Self {
        Self {
            id: q.id.to_string(),
            question: q.question.clone(),
            user_answer: q.user_answer.clone(),
            expected_answer: q.expected_answer.clone(),
            hint: q.hint.clone(),
        }
    }
}

impl From<&OpenQuestionSet> for OpenQuestionSetResponse {
    fn from(set: &OpenQuestionSet) -> Self {
        Self {
            id: set.id.to_string(),
            user_id: set.user_id.to_string(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: match set.level {
                Level::Easy => "easy".to_string(),
                Level::Medium => "medium".to_string(),
                Level::Hard => "hard".to_string(),
            },
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            questions: set
                .questions
                .iter()
                .map(OpenQuestionResponse::from)
                .collect(),
        }
    }
}

impl OpenQuestionSetListResponse {
    pub fn from_sets(sets: Vec<OpenQuestionSet>) -> Self {
        let count = sets.len();
        Self {
            sets: sets.iter().map(OpenQuestionSetResponse::from).collect(),
            count,
        }
    }
}

impl From<&GradedAnswer> for GradedAnswerResponse {
    fn from(g: &GradedAnswer) -> Self {
        Self {
            question_id: g.question_id.clone(),
            grade: match g.grade {
                AnswerGrade::Right => "right".to_string(),
                AnswerGrade::Medium => "medium".to_string(),
                AnswerGrade::Error => "error".to_string(),
            },
            feedback: g.feedback.clone(),
        }
    }
}
