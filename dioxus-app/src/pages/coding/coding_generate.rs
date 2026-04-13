use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::input::Input;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::select::{Select, SelectOption};
use crate::components::ui::spinner::Spinner;
use crate::routes::Route;
use crate::state::auth::use_auth_guard;
use crate::state::coding::CodingExercise;
use dioxus::prelude::*;

pub fn CodingGeneratePage() -> Element {
    let _auth = use_auth_guard();

    let mut language = use_signal(|| "rust".to_string());
    let mut subject = use_signal(String::new);
    let mut level = use_signal(|| "medium".to_string());
    let mut langue = use_signal(|| "eng".to_string());
    let mut loading = use_signal(|| false);
    let mut error: Signal<Option<String>> =
        use_signal(|| None);
    let mut exercise_ctx =
        use_context::<Signal<Option<CodingExercise>>>();
    let nav = navigator();

    let disabled = use_memo(move || (loading)());

    let handle_submit = move |_| {
        loading.set(true);
        error.set(None);

        let req = build_request(
            &language, &subject, &level, &langue,
        );
        let lang = (language)();

        spawn(async move {
            match api::coding::generate_coding_game(&req)
                .await
            {
                Ok(resp) => {
                    exercise_ctx.set(Some(
                        CodingExercise::from((resp, lang)),
                    ));
                    nav.push(Route::CodingPlay {});
                }
                Err(e) => error.set(Some(e)),
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner {
                    title: "Coding Game",
                    span {}
                }
                div { class: "flex flex-col gap-4",
                    Select {
                        id: "coding-lang",
                        label: "Programming Language"
                            .to_string(),
                        options: language_options(),
                        value: language,
                        on_change: move |v: String| {
                            language.set(v)
                        },
                    }
                    Input {
                        input_type: "text",
                        id: "coding-subject",
                        label: "Subject (optional)"
                            .to_string(),
                        placeholder: "e.g. loops, \
                            ownership, async...",
                        value: subject,
                        on_input: move |v: String| {
                            subject.set(v)
                        },
                    }
                    Select {
                        id: "coding-level",
                        label: "Difficulty".to_string(),
                        options: difficulty_options(),
                        value: level,
                        on_change: move |v: String| {
                            level.set(v)
                        },
                    }
                    Select {
                        id: "coding-langue",
                        label: "Exercise Language"
                            .to_string(),
                        options: langue_options(),
                        value: langue,
                        on_change: move |v: String| {
                            langue.set(v)
                        },
                    }
                    if let Some(e) = (error)() {
                        p {
                            class: "text-[var(--color-error)] \
                                text-sm m-0",
                            "{e}"
                        }
                    }
                    if (loading)() {
                        div {
                            class: "flex items-center gap-3 \
                                text-[var(--color-text-secondary)]",
                            Spinner {}
                            span { "Generating..." }
                        }
                    } else {
                        Button {
                            text: "Generate",
                            variant: ButtonVariant::Primary,
                            disabled: disabled,
                            on_click: handle_submit,
                        }
                    }
                }
            }
        }
    }
}

fn build_request(
    language: &Signal<String>,
    subject: &Signal<String>,
    level: &Signal<String>,
    langue: &Signal<String>,
) -> api::coding::GenerateCodingRequest {
    let subj = (subject)();
    let lng = (langue)();

    api::coding::GenerateCodingRequest {
        language: (language)(),
        subject: if subj.trim().is_empty() {
            None
        } else {
            Some(subj)
        },
        level: (level)(),
        langue: if lng == "eng" {
            None
        } else {
            Some(lng)
        },
    }
}

fn language_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("rust", "Rust"),
        SelectOption::new("python", "Python"),
        SelectOption::new("javascript", "JavaScript"),
        SelectOption::new("go", "Go"),
    ]
}

fn difficulty_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ]
}

fn langue_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("eng", "English"),
        SelectOption::new("fr", "Français"),
        SelectOption::new("nl", "Nederlands"),
    ]
}
