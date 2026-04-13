use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::code_editor::{
    get_editor_value, CodeEditor, CodeLanguage,
};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::routes::Route;
use crate::state::auth::use_auth_guard;
use crate::state::coding::CodingExercise;
use dioxus::prelude::*;

const EDITOR_ID: &str = "coding-play-editor";

pub fn CodingPlayPage() -> Element {
    let _auth = use_auth_guard();
    let exercise_ctx =
        use_context::<Signal<Option<CodingExercise>>>();

    let exercise = (exercise_ctx)();

    if exercise.is_none() {
        return rsx! {
            div { class: "home-page",
                HomeTopBar {}
                PageLayout {
                    HeroBanner {
                        title: "Coding Game",
                        span {}
                    }
                    p { "No exercise loaded." }
                    Link { to: Route::CodingGenerate {},
                        "Generate one"
                    }
                }
            }
        };
    }

    let data = exercise.unwrap();
    let snippet_for_check = data.code_snippet.clone();
    let lang = CodeLanguage::from_lang_str(&data.language);

    let mut submitting = use_signal(|| false);
    let mut result: Signal<Option<bool>> =
        use_signal(|| None);
    let mut error: Signal<Option<String>> =
        use_signal(|| None);

    let handle_submit = move |_| {
        submitting.set(true);
        error.set(None);
        result.set(None);

        let expected = snippet_for_check.clone();
        spawn(async move {
            let user_code =
                get_editor_value(EDITOR_ID).await;
            let req = api::coding::CheckCodingRequest {
                expected,
                user_code,
            };
            match api::coding::check_coding_game(&req)
                .await
            {
                Ok(resp) => {
                    result.set(Some(resp.is_correct))
                }
                Err(e) => error.set(Some(e)),
            }
            submitting.set(false);
        });
    };

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner {
                    title: "{data.subject}",
                    span {}
                }
                div { class: "flex flex-col gap-6",
                    div {
                        h3 { class: "text-lg font-semibold \
                            text-[var(--color-text-primary)] \
                            mb-2",
                            "Code to re-write"
                        }
                        pre {
                            class: "p-4 rounded-lg \
                                bg-[var(--glass-bg)] \
                                border border-[var(--glass-border)] \
                                overflow-x-auto",
                            code { "{data.code_snippet}" }
                        }
                    }
                    div {
                        h3 { class: "text-lg font-semibold \
                            text-[var(--color-text-primary)] \
                            mb-2",
                            "Your answer"
                        }
                        CodeEditor {
                            language: lang,
                            initial_code: String::new(),
                            editor_id: EDITOR_ID.to_string(),
                        }
                    }
                    if let Some(e) = (error)() {
                        p {
                            class: "text-[var(--color-error)] text-sm",
                            "{e}"
                        }
                    }
                    if let Some(correct) = (result)() {
                        if correct {
                            p { class: "text-green-500 text-lg font-bold",
                                "✅ Correct!"
                            }
                        } else {
                            p { class: "text-red-500 text-lg font-bold",
                                "❌ Try again"
                            }
                        }
                    }
                    if (submitting)() {
                        div { class: "flex items-center gap-3",
                            Spinner {}
                            span { "Checking..." }
                        }
                    } else {
                        Button {
                            text: "Submit",
                            variant: ButtonVariant::Primary,
                            on_click: handle_submit,
                        }
                    }
                }
            }
        }
    }
}
