// ** quick_form.rs **
// ==> Quick QCM form — file + level + num + lang

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
use crate::components::ui::toast::use_toast;
use crate::domain::qcm_types::QcmSet;
use leptos::callback::UnsyncCallback;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::cell::RefCell;
use std::rc::Rc;

stylance::import_crate_style!(
    style,
    "src/pages/intello/qcm/qcm_quick.module.css"
);

/// Quick QCM form component
#[component]
pub fn QuickQcmForm(
    /// Called with generated QcmSet
    #[prop(into)]
    on_result: Callback<QcmSet>,
) -> impl IntoView {
    let toast = use_toast();
    let loading = RwSignal::new(false);
    let err: RwSignal<Option<String>> =
        RwSignal::new(None);

    // Form field signals
    let level = RwSignal::new("medium".to_string());
    let lang = RwSignal::new("en".to_string());
    let num_q = RwSignal::new("10".to_string());
    let file_count = RwSignal::new(0_usize);
    let files = Rc::new(RefCell::new(Vec::new()));

    let file_cb =
        build_file_cb(files.clone(), file_count);

    let submit = build_submit(SubmitCtx {
        files, level, lang, num_q, err, loading,
        on_result, toast,
    });

    view! {
        <div
            class=style::loading_wrap
            style:display=move || {
                if loading.get() { "flex" }
                else { "none" }
            }
        >
            <Spinner />
            <p class=style::loading_text>
                "Generating questions..."
            </p>
        </div>
        <div
            class=style::form
            style:display=move || {
                if loading.get() { "none" }
                else { "flex" }
            }
        >
            <FileUpload
                on_files=file_cb
                accept=".pdf,.txt,.docx,.pptx"
            />
            <p style:display=move || {
                if file_count.get() > 0 { "block" }
                else { "none" }
            }>
                {move || format!(
                    "{} file(s) selected",
                    file_count.get(),
                )}
            </p>
            <div class=style::row>
                <Select
                    id="level"
                    label="Difficulty"
                    value=level
                    on_change=Callback::new(
                        move |v| level.set(v)
                    )
                    options=level_options()
                />
                <Select
                    id="num_q"
                    label="Questions"
                    value=num_q
                    on_change=Callback::new(
                        move |v| num_q.set(v)
                    )
                    options=num_options()
                />
            </div>
            <Select
                id="lang"
                label="Language"
                value=lang
                on_change=Callback::new(
                    move |v| lang.set(v)
                )
                options=lang_options()
            />
            {move || err.get().map(|e| view! {
                <p style="color:var(--color-error,red)">
                    {e}
                </p>
            })}
            <div class=style::submit_row>
                <button
                    class=btn_class(
                        ButtonVariant::Primary,
                        ButtonSize::Medium,
                    )
                    on:click=submit
                >
                    "Generate"
                </button>
            </div>
        </div>
    }
}

/// Build file change callback
fn build_file_cb(
    files: Rc<RefCell<Vec<web_sys::File>>>,
    count: RwSignal<usize>,
) -> UnsyncCallback<Vec<web_sys::File>> {
    UnsyncCallback::new(
        move |f: Vec<web_sys::File>| {
            count.set(f.len());
            *files.borrow_mut() = f;
        },
    )
}

/// Context for building the submit handler
#[derive(Clone)]
struct SubmitCtx {
    files: Rc<RefCell<Vec<web_sys::File>>>,
    level: RwSignal<String>,
    lang: RwSignal<String>,
    num_q: RwSignal<String>,
    err: RwSignal<Option<String>>,
    loading: RwSignal<bool>,
    on_result: Callback<QcmSet>,
    toast: crate::components::ui::toast::ToastState,
}

/// Build submit click handler
fn build_submit(
    ctx: SubmitCtx,
) -> impl Fn(leptos::ev::MouseEvent) + Clone {
    move |_: leptos::ev::MouseEvent| {
        if ctx.files.borrow().is_empty() {
            ctx.err.set(Some(
                "Upload a file first".into(),
            ));
            return;
        }
        ctx.err.set(None);
        ctx.loading.set(true);

        let form = build_form_data(
            &ctx.level.get(),
            &ctx.lang.get(),
            &ctx.num_q.get(),
            &ctx.files.borrow(),
        );

        let toast = ctx.toast;
        let on_result = ctx.on_result;
        let err = ctx.err;
        let loading = ctx.loading;
        spawn_local(async move {
            match api::intello::generate_quick_qcm(
                &form,
            )
            .await
            {
                Ok(set) => {
                    toast.success(
                        "QCM ready!".into(),
                    );
                    on_result.run(set);
                }
                Err(e) => err.set(Some(e)),
            }
            loading.set(false);
        });
    }
}

/// Build multipart FormData for quick QCM endpoint
fn build_form_data(
    level: &str,
    language: &str,
    num_questions: &str,
    files: &[web_sys::File],
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

    for f in files {
        let _ = form.append_with_blob("files", f);
    }
    form
}
