// ** open_question_player.rs **
// ==> Open question player — textarea answers, AI grading

use crate::api;
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
) -> Element {
    let set_id = set.id.clone();
    let questions = set.questions.clone();
    let total = questions.len();

    // One signal per question for user answer
    let answers: Vec<Signal<String>> = use_hook(|| {
        (0..total)
            .map(|_| Signal::new(String::new()))
            .collect()
    });

    let mut grading = use_signal(|| false);
    let mut grades: Signal<
        Option<CheckAnswersResponse>,
    > = use_signal(|| None);
    let mut error: Signal<Option<String>> =
        use_signal(|| None);

    let qs_c = questions.clone();
    let ans_c = answers.clone();
    let sid = set_id.clone();
    let submit = move |_| {
        grading.set(true);
        error.set(None);
        let req = CheckAnswersRequest {
            set_id: sid.clone(),
            answers: qs_c
                .iter()
                .zip(ans_c.iter())
                .map(|(q, a)| UserAnswerInput {
                    question_id: q.id.clone(),
                    user_answer: (a)(),
                })
                .collect(),
        };
        spawn(async move {
            match api::open_questions::check_answers(
                &req,
            )
            .await
            {
                Ok(resp) => grades.set(Some(resp)),
                Err(e) => error.set(Some(e)),
            }
            grading.set(false);
        });
    };

    rsx! {
        div {
            class: "flex flex-col gap-5 \
                max-w-[700px] mx-auto",
            if let Some(g) = (grades)() {
                {render_grades(&g, on_back)}
            } else {
                {render_questions(
                    &questions,
                    &answers,
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

/// Render question blocks for user input
fn render_questions(
    questions: &[OpenQuestion],
    answers: &[Signal<String>],
) -> Element {
    rsx! {
        for (i, q) in questions.iter().enumerate() {
            div {
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
            for grade in g.grades.iter() {
                div {
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
