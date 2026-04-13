//! Editor for the list of QCM questions inside the create form.

use dioxus::prelude::*;

use crate::qcm::create::{FormFields, FeedbackState};
use crate::qcm::types::CreateQcmQuestionInput;
use crate::ui::{
    Button, ButtonSize, ButtonVariant, Icon, Input,
    SectionDivider, Textarea,
};

/// Maximum number of questions allowed in a set.
pub const MAX_QUESTIONS: usize = 20;

/// Number of wrong-answer fields per question.
pub const WRONG_COUNT: usize = 3;

/// Editable state for a single question (signals only).
#[derive(Clone, PartialEq)]
pub struct QuestionState {
    pub question: Signal<String>,
    pub right_answer: Signal<String>,
    pub wrong_answers: [Signal<String>; WRONG_COUNT],
    pub explanation: Signal<String>,
}

impl Default for QuestionState {
    fn default() -> Self {
        Self {
            question: Signal::new(String::new()),
            right_answer: Signal::new(String::new()),
            wrong_answers: [
                Signal::new(String::new()),
                Signal::new(String::new()),
                Signal::new(String::new()),
            ],
            explanation: Signal::new(String::new()),
        }
    }
}

impl QuestionState {
    /// Whether every text field is non-empty.
    pub fn is_complete(&self) -> bool {
        let txt = (self.question)();
        let right = (self.right_answer)();
        let expl = (self.explanation)();
        let wrongs_ok = self
            .wrong_answers
            .iter()
            .all(|w| !(w)().trim().is_empty());
        !txt.trim().is_empty()
            && !right.trim().is_empty()
            && !expl.trim().is_empty()
            && wrongs_ok
    }

    /// Convert into the API input struct.
    pub fn to_input(&self) -> CreateQcmQuestionInput {
        CreateQcmQuestionInput {
            question: (self.question)().trim().to_string(),
            right_answer: (self.right_answer)()
                .trim()
                .to_string(),
            wrong_answers: self
                .wrong_answers
                .iter()
                .map(|w| (w)().trim().to_string())
                .collect(),
            explanation: (self.explanation)()
                .trim()
                .to_string(),
        }
    }
}

/// Render the "Questions" section plus the "Add" button.
pub fn render_questions_section(
    fields: FormFields,
    feedback: FeedbackState,
) -> Element {
    let mut questions = fields.questions;
    let disabled = feedback.submitting;
    let add_disabled = use_memo(move || {
        (questions)().len() >= MAX_QUESTIONS
    });
    rsx! {
        section { class: "flex flex-col gap-4 pt-4",
            SectionDivider {
                label: "Questions".to_string(),
            }
            for (idx, q) in (questions)().iter().enumerate() {
                QuestionCard {
                    index: idx,
                    question: q.clone(),
                    on_remove: move |_| {
                        remove_question(idx, questions);
                    },
                    can_remove: use_memo(move || {
                        (questions)().len() > 1
                    }),
                    disabled: disabled,
                }
            }
            Button {
                text: "Add Question".to_string(),
                variant: ButtonVariant::Outline,
                size: ButtonSize::Small,
                on_click: move |_| {
                    if (questions)().len() < MAX_QUESTIONS {
                        questions.write().push(
                            QuestionState::default(),
                        );
                    }
                },
                disabled: add_disabled,
            }
        }
    }
}

/// Drop a question by index when more than one remain.
fn remove_question(
    idx: usize,
    mut questions: Signal<Vec<QuestionState>>,
) {
    let len = (questions)().len();
    if len <= 1 {
        return;
    }
    let mut qs = questions.write();
    if idx < qs.len() {
        qs.remove(idx);
    }
}

/// Single-question editor card.
#[component]
fn QuestionCard(
    index: usize,
    question: QuestionState,
    on_remove: EventHandler<()>,
    can_remove: Memo<bool>,
    disabled: Signal<bool>,
) -> Element {
    let mut text = question.question;
    let mut explanation = question.explanation;
    rsx! {
        div { class: "flex flex-col gap-4",
            SectionDivider {
                label: format!("Question {}", index + 1),
            }
            if (can_remove)() {
                button {
                    r#type: "button",
                    class: "self-end text-[var(--color-error)] \
                        text-[0.8rem] cursor-pointer \
                        bg-transparent border-none",
                    onclick: move |_| on_remove.call(()),
                    Icon { icon_name: "Trash" }
                    " Remove"
                }
            }
            Textarea {
                id: format!("q-{}-question", index),
                placeholder: "Enter your question..."
                    .to_string(),
                value: text,
                on_input: move |v| text.set(v),
                rows: 2_u32,
                required: true,
                disabled: disabled,
            }
            AnswerInputs {
                index: index,
                question: question.clone(),
            }
            Textarea {
                id: format!("q-{}-explanation", index),
                placeholder: "Explain the correct answer..."
                    .to_string(),
                value: explanation,
                on_input: move |v| explanation.set(v),
                rows: 2_u32,
                required: true,
                disabled: disabled,
            }
        }
    }
}

/// Grid of correct + wrong answer inputs.
#[component]
fn AnswerInputs(
    index: usize,
    question: QuestionState,
) -> Element {
    let mut right = question.right_answer;
    rsx! {
        div { class: "grid grid-cols-2 gap-4 \
            max-sm:grid-cols-1",
            Input {
                input_type: "text".to_string(),
                id: format!("q-{}-right", index),
                placeholder: "Correct answer".to_string(),
                value: right,
                on_input: move |v| right.set(v),
                required: true,
            }
            for (wi, wrong) in
                question.wrong_answers.iter().enumerate()
            {
                {render_wrong_input(index, wi, *wrong)}
            }
        }
    }
}

/// Render a single wrong-answer input control.
fn render_wrong_input(
    q_index: usize,
    w_index: usize,
    mut signal: Signal<String>,
) -> Element {
    rsx! {
        Input {
            input_type: "text".to_string(),
            id: format!("q-{}-wrong-{}", q_index, w_index),
            placeholder: format!(
                "Wrong answer {}", w_index + 1,
            ),
            value: signal,
            on_input: move |v| signal.set(v),
            required: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_state_default_is_incomplete() {
        let q = QuestionState::default();
        assert!(!q.is_complete());
    }
}
