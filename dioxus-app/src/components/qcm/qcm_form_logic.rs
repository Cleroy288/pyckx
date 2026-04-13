//! QCM form submission logic

use crate::api::study::{
    create_qcm_set, CreateQcmQuestionInput,
    CreateQcmSetRequest,
};
use super::manual_qcm_form::{
    QuestionState,
};
use dioxus::prelude::*;

/// Build the API request from form signals
pub fn build_request(
    name: &Signal<String>,
    description: &Signal<String>,
    level: &Signal<String>,
    language: &Signal<String>,
    subjects: &Signal<Vec<String>>,
    questions: &Signal<Vec<QuestionState>>,
) -> CreateQcmSetRequest {
    CreateQcmSetRequest {
        name: (name)().trim().to_string(),
        description: (description)()
            .trim()
            .to_string(),
        level: (level)(),
        language: (language)(),
        subjects: (subjects)(),
        questions: map_questions(questions),
    }
}

/// Map question signals to API input structs
fn map_questions(
    questions: &Signal<Vec<QuestionState>>,
) -> Vec<CreateQcmQuestionInput> {
    (questions)()
        .iter()
        .map(|q| CreateQcmQuestionInput {
            question: (q.question)()
                .trim()
                .to_string(),
            right_answer: (q.right_answer)()
                .trim()
                .to_string(),
            wrong_answers: q
                .wrong_answers
                .iter()
                .map(|w| (w)().trim().to_string())
                .collect(),
            explanation: (q.explanation)()
                .trim()
                .to_string(),
        })
        .collect()
}

/// Handle the form submission
pub async fn handle_submit(
    request: CreateQcmSetRequest,
    on_success: Option<EventHandler<String>>,
    success_msg: &mut Signal<Option<String>>,
    error_msg: &mut Signal<Option<String>>,
    is_submitting: &mut Signal<bool>,
) {
    match create_qcm_set(request).await {
        Ok(set) => {
            let msg = format!(
                "Created QCM set '{}' \
                    with {} questions!",
                set.name,
                set.questions.len()
            );
            success_msg.set(Some(msg));
            is_submitting.set(false);
            if let Some(cb) = on_success {
                cb.call(set.id);
            }
        }
        Err(e) => {
            error_msg.set(Some(e));
            is_submitting.set(false);
        }
    }
}
