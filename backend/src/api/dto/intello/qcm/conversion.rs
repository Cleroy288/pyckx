//! QCM DTO Conversions

use crate::domain::intello::{Level, QcmQuestion, QcmSet};
use super::response::{QcmSetResponse, QcmQuestionResponse, QcmSetListResponse, QcmSuccessResponse};

impl From<&QcmSet> for QcmSetResponse {
    fn from(set: &QcmSet) -> Self {
        Self {
            id: set.id.clone(),
            user_id: set.user_id.clone(),
            name: set.name.clone(),
            description: set.description.clone(),
            level: match set.level {
                Level::Easy => "easy".to_string(),
                Level::Medium => "medium".to_string(),
                Level::Hard => "hard".to_string(),
            },
            language: set.language.clone(),
            subjects: set.subjects.clone(),
            questions: set.questions.iter().map(QcmQuestionResponse::from).collect(),
        }
    }
}

impl From<&QcmQuestion> for QcmQuestionResponse {
    fn from(q: &QcmQuestion) -> Self {
        Self {
            id: q.id.clone(),
            question: q.question.clone(),
            wrong_answers: q.wrong_answers.clone(),
            right_answer: q.right_answer.clone(),
            explanation: q.explanation.clone(),
        }
    }
}

impl QcmSetListResponse {
    pub fn from_sets(sets: Vec<QcmSet>) -> Self {
        let count = sets.len();
        Self {
            sets: sets.iter().map(QcmSetResponse::from).collect(),
            count,
        }
    }
}

impl QcmSuccessResponse {
    pub fn created(set: &QcmSet) -> Self {
        Self {
            success: true,
            message: "QCM set created successfully".to_string(),
            set: Some(QcmSetResponse::from(set)),
        }
    }

    pub fn updated(set: &QcmSet) -> Self {
        Self {
            success: true,
            message: "QCM set updated successfully".to_string(),
            set: Some(QcmSetResponse::from(set)),
        }
    }
}
