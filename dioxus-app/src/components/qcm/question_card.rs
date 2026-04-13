//! QuestionCard — single question editor card

use crate::components::ui::icon::Icon;
use crate::components::ui::input::Input;
use crate::components::ui::section_divider::{
    SectionDivider,
};
use crate::components::ui::textarea::Textarea;
use dioxus::prelude::*;

use super::manual_qcm_form::{
    QuestionState,
};

/// Single question editing card
#[component]
pub fn QuestionCard(
    index: usize,
    question: QuestionState,
    on_remove: EventHandler<()>,
    can_remove: Memo<bool>,
    disabled: Signal<bool>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4",
            SectionDivider {
                label: format!(
                    "Question {}", index + 1,
                ),
            }
            if (can_remove)() {
                button {
                    r#type: "button",
                    class: "inline-flex items-center \
                        gap-1 self-end bg-transparent \
                        border-none py-1 px-2 \
                        cursor-pointer \
                        text-[var(--color-error)] \
                        text-[0.8rem] opacity-70 \
                        hover:opacity-100",
                    onclick: move |_| {
                        on_remove.call(())
                    },
                    Icon { icon_name: "Trash" }
                    " Remove"
                }
            }
            Textarea {
                id: format!("q-{}-question", index),
                placeholder: "Enter your question...",
                value: question.question,
                on_input: move |val: String| {
                    question.question.set(val)
                },
                rows: 2,
                required: true,
                disabled: disabled,
            }
            AnswerGrid {
                index: index,
                question: question.clone(),
            }
            Textarea {
                id: format!(
                    "q-{}-explanation", index,
                ),
                placeholder: "Explain the correct \
                    answer...",
                value: question.explanation,
                on_input: move |val: String| {
                    question.explanation.set(val)
                },
                rows: 2,
                required: true,
                disabled: disabled,
            }
        }
    }
}

/// Grid of answer inputs (correct + wrong)
#[component]
fn AnswerGrid(
    index: usize,
    question: QuestionState,
) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-2 gap-4 \
                max-sm:grid-cols-1",
            Input {
                input_type: "text",
                id: format!("q-{}-right", index),
                placeholder: "Correct answer",
                value: question.right_answer,
                on_input: move |val: String| {
                    question.right_answer.set(val)
                },
                required: true,
            }
            for (wi, wrong) in question
                .wrong_answers.iter().enumerate()
            {
                Input {
                    input_type: "text",
                    id: format!(
                        "q-{}-wrong-{}", index, wi,
                    ),
                    placeholder: format!(
                        "Wrong answer {}", wi + 1,
                    ),
                    value: *wrong,
                    on_input: {
                        let mut w = *wrong;
                        move |val: String| w.set(val)
                    },
                    required: true,
                }
            }
        }
    }
}
