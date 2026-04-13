//! Open question player — textarea answers, AI grading

use crate::components::ui::button::Button;
use crate::components::ui::spinner::Spinner;
use crate::components::ui::textarea::Textarea;
use crate::domain::open_question_types::*;
use dioxus::prelude::*;

/// Open question player — answer all then submit
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
    let total = questions.len();

    let answers: Vec<Signal<String>> = use_hook(|| {
        (0..total)
            .map(|_| Signal::new(String::new()))
            .collect()
    });

    let submit = build_submit(
        set.id.clone(),
        questions.clone(),
        answers.clone(),
        on_check,
    );

    rsx! {
        div {
            class: "flex flex-col gap-5 \
                max-w-[700px] mx-auto",
            if let Some(g) = (grades)() {
                {render_grades(&g, on_back)}
            } else {
                {render_questions(
                    &questions, &answers,
                )}
                if let Some(e) = (error)() {
                    p {
                        class: "text-[var(--color-error)] \
                            text-sm",
                        "{e}"
                    }
                }
                if (grading)() {
                    Spinner {}
                } else {
                    Button {
                        text: "Submit Answers",
                        on_click: submit,
                    }
                }
            }
        }
    }
}

/// Build the submit closure from question data
fn build_submit(
    set_id: String,
    questions: Vec<OpenQuestion>,
    answers: Vec<Signal<String>>,
    on_check: EventHandler<CheckAnswersRequest>,
) -> impl Fn(Event<MouseData>) {
    move |_| {
        let req = CheckAnswersRequest {
            set_id: set_id.clone(),
            answers: questions
                .iter()
                .zip(answers.iter())
                .map(|(q, a)| UserAnswerInput {
                    question_id: q.id.clone(),
                    user_answer: (a)(),
                })
                .collect(),
        };
        on_check.call(req);
    }
}

/// Render question blocks for user input
fn render_questions(
    questions: &[OpenQuestion],
    answers: &[Signal<String>],
) -> Element {
    rsx! {
        for (i, q) in questions.iter().enumerate() {
            div {
                key: "{i}",
                class: "p-4 bg-[var(--glass-bg)] \
                    backdrop-blur-[20px] \
                    border border-[var(--color-border)]",
                h3 {
                    class: "m-0 mb-2 text-base \
                        font-semibold \
                        text-[var(--color-text-primary)]",
                    "{i + 1}. {q.question}"
                }
                if let Some(h) = &q.hint {
                    p {
                        class: "m-0 mb-2 \
                            text-[0.8125rem] \
                            text-[var(--color-text-secondary)] \
                            italic",
                        "Hint: {h}"
                    }
                }
                Textarea {
                    id: format!("oq-{}", i),
                    placeholder: "Your answer...",
                    value: answers[i],
                    on_input: {
                        let mut sig = answers[i];
                        move |v: String| sig.set(v)
                    },
                }
            }
        }
    }
}

/// Render graded results
fn render_grades(
    g: &CheckAnswersResponse,
    on_back: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-4",
            h2 { "Results" }
            for (index, grade) in g.grades.iter().enumerate() {
                div {
                    key: "{index}",
                    class: "p-4 bg-[var(--glass-bg)] \
                        backdrop-blur-[20px] \
                        border \
                        border-[var(--color-border)]",
                    span {
                        class: "font-bold \
                            text-[var(--color-primary)]",
                        "{grade.grade}"
                    }
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
