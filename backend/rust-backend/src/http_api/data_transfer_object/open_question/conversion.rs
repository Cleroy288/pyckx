//! Open Question DTO Conversions

use super::response::{
    GradedAnswerResponse, OpenQuestionResponse, OpenQuestionSetListResponse,
    OpenQuestionSetResponse,
};
use crate::services::games::shared::types::{
    AnswerGrade, GradedAnswer,
};
use crate::services::{Level, OpenQuestion, OpenQuestionSet};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{
        Level, QuestionId, SetId,
    };
    use crate::infra::user::UserId;

    /// Build a test OpenQuestion
    fn test_question() -> OpenQuestion {
        OpenQuestion {
            id: QuestionId::from_string("oq-1".into()),
            question: "Explain gravity".into(),
            user_answer: "Force of attraction".into(),
            expected_answer: Some("Newton's law".into()),
            hint: Some("Think physics".into()),
        }
    }

    /// Build a test OpenQuestionSet
    fn test_set() -> OpenQuestionSet {
        OpenQuestionSet {
            id: SetId::from_string("oset-1".into()),
            user_id: UserId::from("user-1"),
            name: "Physics".into(),
            description: "Basics".into(),
            level: Level::Hard,
            language: "en".into(),
            subjects: vec!["physics".into()],
            questions: vec![test_question()],
        }
    }

    #[test]
    fn test_open_question_response_maps_fields() {
        // arrange
        let q = test_question();

        // act
        let resp = OpenQuestionResponse::from(&q);

        // assert
        assert_eq!(resp.id, "oq-1");
        assert_eq!(resp.question, "Explain gravity");
        assert_eq!(
            resp.expected_answer,
            Some("Newton's law".into())
        );
    }

    #[test]
    fn test_open_question_set_response_level() {
        // arrange
        let set = test_set();

        // act
        let resp = OpenQuestionSetResponse::from(&set);

        // assert
        assert_eq!(resp.level, "hard");
    }

    #[test]
    fn test_open_question_set_response_fields() {
        // arrange
        let set = test_set();

        // act
        let resp = OpenQuestionSetResponse::from(&set);

        // assert
        assert_eq!(resp.id, "oset-1");
        assert_eq!(resp.name, "Physics");
        assert_eq!(resp.questions.len(), 1);
    }

    #[test]
    fn test_open_question_set_list_count() {
        // arrange
        let sets = vec![test_set()];

        // act
        let resp = OpenQuestionSetListResponse::from_sets(sets);

        // assert
        assert_eq!(resp.count, 1);
    }

    #[test]
    fn test_graded_answer_right() {
        // arrange
        let ga = GradedAnswer {
            question_id: "q-1".into(),
            grade: AnswerGrade::Right,
            feedback: "Correct!".into(),
        };

        // act
        let resp = GradedAnswerResponse::from(&ga);

        // assert
        assert_eq!(resp.grade, "right");
    }

    #[test]
    fn test_graded_answer_medium() {
        // arrange
        let ga = GradedAnswer {
            question_id: "q-2".into(),
            grade: AnswerGrade::Medium,
            feedback: "Partially".into(),
        };

        // act
        let resp = GradedAnswerResponse::from(&ga);

        // assert
        assert_eq!(resp.grade, "medium");
    }

    #[test]
    fn test_graded_answer_error() {
        // arrange
        let ga = GradedAnswer {
            question_id: "q-3".into(),
            grade: AnswerGrade::Error,
            feedback: "Wrong".into(),
        };

        // act
        let resp = GradedAnswerResponse::from(&ga);

        // assert
        assert_eq!(resp.grade, "error");
        assert_eq!(resp.feedback, "Wrong");
    }
}
