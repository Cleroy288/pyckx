//! Coding play view — small presentational pieces and
//! the reactive status the page pumps into them.

use dioxus::prelude::*;

use crate::coding::state::CodingExercise;
use crate::home::HomeTopBar;
use crate::routes::Route;
use crate::ui::{
    Button, ButtonVariant, CodeEditor, CodeLanguage,
    HeroBanner, PageLayout, Spinner,
};

/// Reactive bag tracking submission progress and verdict.
#[derive(Clone, Copy, PartialEq)]
pub struct StatusState {
    pub submitting: Signal<bool>,
    pub result: Signal<Option<bool>>,
    pub error: Signal<Option<String>>,
}

impl StatusState {
    /// Build an idle status (no submission in flight).
    pub fn new() -> Self {
        Self {
            submitting: use_signal(|| false),
            result: use_signal(|| None),
            error: use_signal(|| None),
        }
    }

    /// Mark a submission as starting (clears stale state).
    pub fn start(&mut self) {
        self.submitting.set(true);
        self.error.set(None);
        self.result.set(None);
    }

    /// Mark a submission as finished with a verdict.
    pub fn finish(&mut self, verdict: Option<bool>) {
        self.result.set(verdict);
        self.submitting.set(false);
    }

    /// Mark a submission as failed with an error message.
    pub fn fail(&mut self, msg: String) {
        self.error.set(Some(msg));
        self.submitting.set(false);
    }
}

/// Empty-state page shown when no exercise is loaded.
#[component]
pub fn EmptyExercise() -> Element {
    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner { title: "Coding Game", span {} }
                p { "No exercise loaded." }
                Link { to: Route::CodingGenerate {},
                    "Generate one"
                }
            }
        }
    }
}

/// Main body: prompt snippet, editor, verdict, submit button.
#[component]
pub fn ExerciseBody(
    exercise: CodingExercise,
    editor_id: String,
    status: StatusState,
    on_submit: EventHandler<MouseEvent>,
) -> Element {
    let lang = CodeLanguage::from_lang_str(&exercise.language);
    rsx! {
        div { class: "flex flex-col gap-6",
            PromptBlock { snippet: exercise.code_snippet }
            AnswerBlock {
                language: lang,
                editor_id: editor_id,
            }
            ErrorLine { error: status.error }
            Verdict { result: status.result }
            SubmitArea {
                submitting: status.submitting,
                on_click: on_submit,
            }
        }
    }
}

#[component]
fn PromptBlock(snippet: String) -> Element {
    rsx! {
        div {
            h3 {
                class: "text-lg font-semibold \
                    text-[var(--color-text-primary)] mb-2",
                "Code to re-write"
            }
            pre {
                class: "p-4 rounded-lg \
                    bg-[var(--glass-bg)] \
                    border border-[var(--glass-border)] \
                    overflow-x-auto",
                code { "{snippet}" }
            }
        }
    }
}

#[component]
fn AnswerBlock(
    language: CodeLanguage,
    editor_id: String,
) -> Element {
    rsx! {
        div {
            h3 {
                class: "text-lg font-semibold \
                    text-[var(--color-text-primary)] mb-2",
                "Your answer"
            }
            CodeEditor {
                language: language,
                initial_code: String::new(),
                editor_id: editor_id,
            }
        }
    }
}

#[component]
fn ErrorLine(error: Signal<Option<String>>) -> Element {
    let Some(msg) = (error)() else {
        return rsx! {};
    };
    rsx! {
        p { class: "text-[var(--color-error)] text-sm",
            "{msg}"
        }
    }
}

#[component]
fn Verdict(result: Signal<Option<bool>>) -> Element {
    let Some(correct) = (result)() else {
        return rsx! {};
    };
    if correct {
        rsx! {
            p { class: "text-green-500 text-lg font-bold",
                "✅ Correct!"
            }
        }
    } else {
        rsx! {
            p { class: "text-red-500 text-lg font-bold",
                "❌ Try again"
            }
        }
    }
}

#[component]
fn SubmitArea(
    submitting: Signal<bool>,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    if (submitting)() {
        return rsx! {
            div { class: "flex items-center gap-3",
                Spinner {}
                span { "Checking..." }
            }
        };
    }
    rsx! {
        Button {
            text: "Submit",
            variant: ButtonVariant::Primary,
            on_click: move |e| on_click.call(e),
        }
    }
}
