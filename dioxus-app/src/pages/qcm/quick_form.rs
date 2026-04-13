//! Quick QCM form — file + level + num + lang

use super::quick_form_options::{
    lang_options, level_options, num_options,
};
use crate::api;
use crate::components::ui::button::{
    btn_class, ButtonSize, ButtonVariant,
};
use crate::components::ui::file_upload::FileUpload;
use crate::components::ui::select::Select;
use crate::components::ui::spinner::Spinner;
use crate::domain::qcm_types::QcmSet;
use dioxus::prelude::*;
/// Quick QCM form component
#[component]
pub fn QuickQcmForm(
    /// Called with generated QcmSet
    on_result: EventHandler<QcmSet>,
) -> Element {
    let mut loading = use_signal(|| false);
    let mut err: Signal<Option<String>> =
        use_signal(|| None);

    // Form field signals
    let mut level =
        use_signal(|| "medium".to_string());
    let mut lang = use_signal(|| "en".to_string());
    let mut num_q = use_signal(|| "10".to_string());
    let mut file_count = use_signal(|| 0_usize);
    let mut file_names: Signal<Vec<String>> =
        use_signal(Vec::new);

    let file_cb = move |names: Vec<String>| {
        file_count.set(names.len());
        file_names.set(names);
    };

    let submit = move |_: Event<MouseData>| {
        if (file_names)().is_empty() {
            err.set(Some(
                "Upload a file first".into(),
            ));
            return;
        }
        err.set(None);
        loading.set(true);

        let form = build_form_data(
            &(level)(),
            &(lang)(),
            &(num_q)(),
            &(file_names)(),
        );

        spawn(async move {
            match api::study::generate_quick_qcm(
                &form,
            )
            .await
            {
                Ok(set) => on_result.call(set),
                Err(e) => err.set(Some(e)),
            }
            loading.set(false);
        });
    };

    rsx! {
        // Loading overlay
        div {
            class: "items-center justify-center \
                flex-col gap-4",
            style: if (loading)() { "display:flex" }
                else { "display:none" },
            Spinner {}
            p {
                class: "text-[var(--color-text-secondary)]",
                "Generating questions..."
            }
        }
        // Form
        div {
            class: "flex flex-col gap-4",
            style: if (loading)() { "display:none" }
                else { "display:flex" },
            FileUpload {
                on_files: file_cb,
                accept: ".pdf,.txt,.docx,.pptx",
            }
            if (file_count)() > 0 {
                p {
                    "{(file_count)()} file(s) selected"
                }
            }
            div {
                class: "flex gap-4",
                Select {
                    id: "level",
                    label: "Difficulty",
                    value: level,
                    on_change: move |v: String| {
                        level.set(v);
                    },
                    options: level_options(),
                }
                Select {
                    id: "num_q",
                    label: "Questions",
                    value: num_q,
                    on_change: move |v: String| {
                        num_q.set(v);
                    },
                    options: num_options(),
                }
            }
            Select {
                id: "lang",
                label: "Language",
                value: lang,
                on_change: move |v: String| {
                    lang.set(v);
                },
                options: lang_options(),
            }
            if let Some(e) = (err)() {
                p {
                    class: "text-[var(--color-error,red)]",
                    "{e}"
                }
            }
            div {
                class: "flex justify-end",
                button {
                    class: "{btn_class(ButtonVariant::Primary, ButtonSize::Medium)}",
                    onclick: submit,
                    "Generate"
                }
            }
        }
    }
}

/// Build multipart FormData for quick QCM endpoint
///
/// NOTE: Dioxus FileEngine provides filenames only.
/// File content upload requires rework with
/// Dioxus file handling. For now, metadata is sent
/// and filenames are appended as strings.
fn build_form_data(
    level: &str,
    language: &str,
    num_questions: &str,
    file_names: &[String],
) -> web_sys::FormData {
    let form = web_sys::FormData::new().unwrap();

    // Metadata as JSON blob
    let meta = format!(
        r#"{{"level":"{level}","language":"{language}","num_questions":{num_questions}}}"#,
    );
    let blob = web_sys::Blob::new_with_str_sequence(
        &js_sys::Array::of1(&meta.into()),
    )
    .unwrap();
    let _ = form.append_with_blob_and_filename(
        "metadata", &blob, "metadata.json",
    );

    // TODO: File upload needs rework for Dioxus —
    // FileEngine gives names, not File objects.
    for name in file_names {
        let _ =
            form.append_with_str("files", name);
    }
    form
}
