//! Quick QCM page — upload a doc, generate, play.

use dioxus::prelude::*;

use crate::auth::use_auth_guard;
use crate::home::HomeTopBar;
use crate::qcm::api;
use crate::qcm::player::QcmPlayer;
use crate::qcm::types::QcmSet;
use crate::ui::{
    btn_class, Button, ButtonSize, ButtonVariant,
    FileUpload, HeroBanner, PageLayout, Select,
    SelectOption, Spinner,
};

const TITLE: &str = "Quick QCM";
const BACK_PATH: &str = "/qcm";
const ACCEPT: &str = ".pdf,.txt,.docx,.pptx";
const NEED_FILE: &str = "Upload a file first";

/// Quick QCM page — form first, then in-place player.
pub fn QcmQuickPage() -> Element {
    let _auth = use_auth_guard();
    let mut set: Signal<Option<QcmSet>> =
        use_signal(|| None);
    let on_back = move |_| set.set(None);
    let on_result =
        move |new_set: QcmSet| set.set(Some(new_set));

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner { title: TITLE,
                {render_back_button()}
            }
            if let Some(s) = set() {
                QcmPlayer { set: s, on_back: on_back }
            } else {
                QuickForm { on_result: on_result }
            }
        }
    }
}

/// Render the "back to QCM list" button in the header.
fn render_back_button() -> Element {
    let nav = navigator();
    rsx! {
        Button {
            text: "Back".to_string(),
            variant: ButtonVariant::Outline,
            on_click: move |_| { nav.push(BACK_PATH); },
        }
    }
}

/// Inline form to generate a one-shot QCM from a file.
#[component]
fn QuickForm(
    /// Called with the freshly generated set.
    on_result: EventHandler<QcmSet>,
) -> Element {
    let fields = use_quick_fields();
    let state = use_quick_state();
    let on_files = make_files_handler(fields.files);
    let on_submit = make_submit(fields, state, on_result);

    rsx! {
        if (state.loading)() {
            div {
                class: "items-center justify-center \
                    flex flex-col gap-4",
                Spinner {}
                p { class: "text-[var(--color-text-secondary)]",
                    "Generating questions..."
                }
            }
        } else {
            {render_form(
                fields, state, on_files, on_submit,
            )}
        }
    }
}

/// Aggregates every field signal of the quick form.
#[derive(Clone, Copy)]
struct QuickFields {
    level: Signal<String>,
    lang: Signal<String>,
    num_q: Signal<String>,
    files: Signal<Vec<String>>,
}

/// Reactive UI state for submission feedback.
#[derive(Clone, Copy)]
struct QuickState {
    loading: Signal<bool>,
    err: Signal<Option<String>>,
}

/// Allocate every field signal with default values.
fn use_quick_fields() -> QuickFields {
    QuickFields {
        level: use_signal(|| "medium".to_string()),
        lang: use_signal(|| "en".to_string()),
        num_q: use_signal(|| "10".to_string()),
        files: use_signal(Vec::new),
    }
}

/// Allocate the loading + error signals.
fn use_quick_state() -> QuickState {
    QuickState {
        loading: use_signal(|| false),
        err: use_signal(|| None),
    }
}

/// Build the file-selection callback.
fn make_files_handler(
    mut files: Signal<Vec<String>>,
) -> impl FnMut(Vec<String>) + 'static {
    move |names| files.set(names)
}

/// Build the submit handler that calls the API.
fn make_submit(
    fields: QuickFields,
    state: QuickState,
    on_result: EventHandler<QcmSet>,
) -> impl FnMut(Event<MouseData>) + 'static {
    let QuickState { mut loading, mut err } = state;
    move |_| {
        if (fields.files)().is_empty() {
            err.set(Some(NEED_FILE.into()));
            return;
        }
        err.set(None);
        loading.set(true);
        let form = build_quick_form(&fields);
        spawn(async move {
            match api::generate_quick_qcm(&form).await {
                Ok(set) => on_result.call(set),
                Err(e) => err.set(Some(e)),
            }
            loading.set(false);
        });
    }
}

/// Render the form body (selectors + submit button).
fn render_form(
    fields: QuickFields,
    state: QuickState,
    on_files: impl FnMut(Vec<String>) + 'static,
    on_submit: impl FnMut(Event<MouseData>) + 'static,
) -> Element {
    let mut level = fields.level;
    let mut lang = fields.lang;
    let mut num_q = fields.num_q;
    rsx! {
        div { class: "flex flex-col gap-4",
            FileUpload {
                on_files: on_files,
                accept: ACCEPT.to_string(),
            }
            div { class: "flex gap-4",
                Select {
                    id: "level",
                    label: "Difficulty".to_string(),
                    value: level,
                    on_change: move |v| level.set(v),
                    options: level_options(),
                }
                Select {
                    id: "num_q",
                    label: "Questions".to_string(),
                    value: num_q,
                    on_change: move |v| num_q.set(v),
                    options: num_options(),
                }
            }
            Select {
                id: "lang",
                label: "Language".to_string(),
                value: lang,
                on_change: move |v| lang.set(v),
                options: lang_options(),
            }
            if let Some(e) = (state.err)() {
                p { class: "text-[var(--color-error)]",
                    "{e}"
                }
            }
            div { class: "flex justify-end",
                button {
                    class: "{btn_class(\
                        ButtonVariant::Primary, \
                        ButtonSize::Medium)}",
                    onclick: on_submit,
                    "Generate"
                }
            }
        }
    }
}

/// Build the multipart form body for the quick endpoint.
fn build_quick_form(
    fields: &QuickFields,
) -> web_sys::FormData {
    let form = web_sys::FormData::new().unwrap();
    let meta = serde_json::json!({
        "level": (fields.level)(),
        "language": (fields.lang)(),
        "num_questions": (fields.num_q)()
            .parse::<u32>()
            .unwrap_or(10),
    });
    let json: wasm_bindgen::JsValue =
        meta.to_string().into();
    let arr = js_sys::Array::of1(&json);
    let blob =
        web_sys::Blob::new_with_str_sequence(&arr)
            .unwrap();
    let _ = form.append_with_blob_and_filename(
        "metadata", &blob, "metadata.json",
    );
    for name in (fields.files)() {
        let _ = form.append_with_str("files", &name);
    }
    form
}

/// Difficulty select options.
fn level_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("easy", "Easy"),
        SelectOption::new("medium", "Medium"),
        SelectOption::new("hard", "Hard"),
    ]
}

/// Question-count select options.
fn num_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("5", "5"),
        SelectOption::new("10", "10"),
        SelectOption::new("15", "15"),
        SelectOption::new("20", "20"),
    ]
}

/// Language select options.
fn lang_options() -> Vec<SelectOption> {
    vec![
        SelectOption::new("en", "English"),
        SelectOption::new("fr", "Francais"),
        SelectOption::new("es", "Espanol"),
    ]
}
