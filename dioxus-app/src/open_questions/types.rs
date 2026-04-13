//! Open-question domain re-exports.
//!
//! Single import surface for every callsite inside this
//! module. The canonical type definitions live in
//! `crate::domain::open_question_types`.

pub use crate::domain::open_question_types::{
    CheckAnswersRequest, CheckAnswersResponse,
    CreateOpenQuestionResponse, GradedAnswer, OpenQuestion,
    OpenQuestionSet, OpenQuestionSetListResponse,
    UserAnswerInput,
};

/// Build the grading request from a set, the questions, and
/// one user answer per question (same order as `questions`).
///
/// Pure function — no side effects, easy to test.
pub fn build_check_request(
    set_id: &str,
    questions: &[OpenQuestion],
    answers: &[String],
) -> CheckAnswersRequest {
    let pairs = questions.iter().zip(answers.iter());
    let user_answers = pairs
        .map(|(q, a)| UserAnswerInput {
            question_id: q.id.clone(),
            user_answer: a.clone(),
        })
        .collect();
    CheckAnswersRequest {
        set_id: set_id.to_string(),
        answers: user_answers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_question(id: &str) -> OpenQuestion {
        OpenQuestion {
            id: id.to_string(),
            question: "Q?".to_string(),
            user_answer: String::new(),
            expected_answer: None,
            hint: None,
        }
    }

    #[test]
    fn test_build_check_request_pairs_in_order() {
        let questions =
            vec![fake_question("a"), fake_question("b")];
        let answers =
            vec!["one".to_string(), "two".to_string()];

        let req = build_check_request(
            "set-1", &questions, &answers,
        );

        assert_eq!(req.set_id, "set-1");
        assert_eq!(req.answers.len(), 2);
        assert_eq!(req.answers[0].question_id, "a");
        assert_eq!(req.answers[1].user_answer, "two");
    }

    #[test]
    fn test_build_check_request_empty_inputs() {
        let req = build_check_request("s", &[], &[]);
        assert!(req.answers.is_empty());
    }

    #[test]
    fn test_build_check_request_truncates_to_shortest() {
        let questions = vec![fake_question("a")];
        let answers =
            vec!["x".to_string(), "y".to_string()];

        let req = build_check_request(
            "s", &questions, &answers,
        );

        assert_eq!(req.answers.len(), 1);
    }
}
