//! QCM DTO Conversions

use super::response::{
    QcmQuestionResponse, QcmSetListResponse, QcmSetResponse, QcmSuccessResponse,
};
use crate::services::{Level, QcmQuestion, QcmSet};

impl From<&QcmSet> for QcmSetResponse {
    fn from(set: &QcmSet) -> Self {
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
                .map(QcmQuestionResponse::from)
                .collect(),
        }
    }
}

impl From<&QcmQuestion> for QcmQuestionResponse {
    fn from(q: &QcmQuestion) -> Self {
        Self {
            id: q.id.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{QuestionId, SetId};
    use crate::infra::user::UserId;

    /// Build a test QcmQuestion
    fn test_question() -> QcmQuestion {
        QcmQuestion {
            id: QuestionId::from_string("q-1".into()),
            question: "What is 2+2?".into(),
            wrong_answers: vec![
                "3".into(),
                "5".into(),
                "6".into(),
            ],
            right_answer: "4".into(),
            explanation: "Basic math".into(),
        }
    }

    /// Build a test QcmSet
    fn test_set() -> QcmSet {
        QcmSet {
            id: SetId::from_string("set-1".into()),
            user_id: UserId::from("user-1"),
            name: "Math Quiz".into(),
            description: "A test quiz".into(),
            level: Level::Easy,
            language: "en".into(),
            subjects: vec!["math".into()],
            questions: vec![test_question()],
        }
    }

    #[test]
    fn test_qcm_question_response_from_maps_id() {
        // arrange
        let q = test_question();

        // act
        let resp = QcmQuestionResponse::from(&q);

        // assert
        assert_eq!(resp.id, "q-1");
    }

    #[test]
    fn test_qcm_question_response_from_maps_fields() {
        // arrange
        let q = test_question();

        // act
        let resp = QcmQuestionResponse::from(&q);

        // assert
        assert_eq!(resp.question, "What is 2+2?");
        assert_eq!(resp.right_answer, "4");
        assert_eq!(resp.wrong_answers.len(), 3);
    }

    #[test]
    fn test_qcm_set_response_from_maps_level() {
        // arrange
        let set = test_set();

        // act
        let resp = QcmSetResponse::from(&set);

        // assert
        assert_eq!(resp.level, "easy");
    }

    #[test]
    fn test_qcm_set_response_from_maps_fields() {
        // arrange
        let set = test_set();

        // act
        let resp = QcmSetResponse::from(&set);

        // assert
        assert_eq!(resp.id, "set-1");
        assert_eq!(resp.name, "Math Quiz");
        assert_eq!(resp.questions.len(), 1);
    }

    #[test]
    fn test_qcm_set_list_from_sets_count() {
        // arrange
        let sets = vec![test_set(), test_set()];

        // act
        let resp = QcmSetListResponse::from_sets(sets);

        // assert
        assert_eq!(resp.count, 2);
    }

    #[test]
    fn test_qcm_success_created_message() {
        // arrange
        let set = test_set();

        // act
        let resp = QcmSuccessResponse::created(&set);

        // assert
        assert!(resp.success);
        assert_eq!(
            resp.message,
            "QCM set created successfully"
        );
        assert!(resp.set.is_some());
    }

    #[test]
    fn test_qcm_success_updated_message() {
        // arrange
        let set = test_set();

        // act
        let resp = QcmSuccessResponse::updated(&set);

        // assert
        assert!(resp.success);
        assert_eq!(
            resp.message,
            "QCM set updated successfully"
        );
    }
}
