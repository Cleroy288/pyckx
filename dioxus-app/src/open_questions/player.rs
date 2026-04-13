//! Open-question player — answer all, then submit for AI.
//!
//! Reads top-down: the public component owns the answers
//! signal and delegates rendering to two helpers (one for
//! the question form, one for the graded results screen).

use super::types::{
    build_check_request, CheckAnswersRequest,
    CheckAnswersResponse, OpenQuestion, OpenQuestionSet,
};
use crate::ui::{Button, Spinner, Textarea};
use dioxus::prelude::*;

/// Player for an open-question set: shows questions,
/// collects free-form answers, then displays AI grades.
#[component]
pub fn OpenQuestionPlayer(
    set: OpenQuestionSet,
    on_back: EventHandler<()>,
    on_check: EventHandler<CheckAnswersRequest>,
    grades: Signal<Option<CheckAnswersResponse>>,
    grading: Signal<bool>,
    error: Signal<Option<String>>,
) -> Element {
    let questions = set.questions.clone();
    let answers =
        use_signal(|| vec![String::new(); questions.len()]);
    let submit = submit_handler(
        set.id, questions.clone(), answers, on_check,
    );

    rsx! {
        div { class: PLAYER_CLS,
            if let Some(g) = grades() {
                {render_results(g, on_back)}
            } else {
                {render_form(
                    questions, answers, error,
                    grading, submit,
                )}
            }
        }
    }
}

/// Build the submit closure: zip questions+answers, call API.
fn submit_handler(
    set_id: String,
    questions: Vec<OpenQuestion>,
    answers: Signal<Vec<String>>,
    on_check: EventHandler<CheckAnswersRequest>,
) -> impl Fn(Event<MouseData>) {
    move |_| {
        let req = build_check_request(
            &set_id, &questions, &answers.read(),
        );
        on_check.call(req);
    }
}

/// Render the answer form: questions, optional error, footer.
fn render_form(
    questions: Vec<OpenQuestion>,
    answers: Signal<Vec<String>>,
    error: Signal<Option<String>>,
    grading: Signal<bool>,
    submit: impl Fn(Event<MouseData>) + 'static,
) -> Element {
    rsx! {
        for (idx, q) in questions.iter().enumerate() {
            div { key: "{idx}", class: CARD_CLS,
                h3 { class: H3_CLS, "{idx + 1}. {q.question}" }
                if let Some(h) = &q.hint {
                    p { class: HINT_CLS, "Hint: {h}" }
                }
                {answer_input(idx, answers)}
            }
        }
        if let Some(msg) = error() {
            p { class: ERROR_CLS, "{msg}" }
        }
        if grading() {
            Spinner {}
        } else {
            Button { text: "Submit Answers", on_click: submit }
        }
    }
}

/// One textarea bound to `answers[idx]` via a derived signal.
fn answer_input(
    idx: usize,
    mut answers: Signal<Vec<String>>,
) -> Element {
    let value = use_signal(move || {
        answers.read().get(idx).cloned().unwrap_or_default()
    });
    let on_input = move |v: String| {
        if let Some(slot) = answers.write().get_mut(idx) {
            *slot = v;
        }
    };
    rsx! {
        Textarea {
            id: format!("oq-{}", idx),
            placeholder: "Your answer...",
            value: value,
            on_input: on_input,
        }
    }
}

/// Render the graded results screen with a back button.
fn render_results(
    response: CheckAnswersResponse,
    on_back: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { "Results" }
            for (idx, grade) in response.grades.iter().enumerate() {
                div { key: "{idx}", class: CARD_CLS,
                    span { class: GRADE_CLS, "{grade.grade}" }
                    p { "{grade.feedback}" }
                }
            }
            Button {
                text: "Back",
                on_click: move |_| on_back.call(()),
            }
        }
    }
}

const PLAYER_CLS: &str =
    "flex flex-col gap-5 max-w-[700px] mx-auto";

const CARD_CLS: &str = "p-4 bg-[var(--glass-bg)] \
    backdrop-blur-[20px] border border-[var(--color-border)]";

const H3_CLS: &str = "m-0 mb-2 text-base font-semibold \
    text-[var(--color-text-primary)]";

const HINT_CLS: &str = "m-0 mb-2 text-[0.8125rem] \
    text-[var(--color-text-secondary)] italic";

const ERROR_CLS: &str =
    "text-[var(--color-error)] text-sm";

const GRADE_CLS: &str =
    "font-bold text-[var(--color-primary)]";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_constants_are_non_empty() {
        assert!(!PLAYER_CLS.is_empty());
        assert!(!CARD_CLS.is_empty());
    }
}
