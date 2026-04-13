//! Generic "generate a set" page.

use crate::components::top_bar::HomeTopBar;
use crate::game_engine::generate_form::{
    GenerateForm, GenerateFormData,
};
use crate::state::auth::use_auth_guard;
use crate::ui::{use_toast, HeroBanner, PageLayout};
use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Build a multipart `FormData` for the AI generation request.
pub fn build_form_data(
    data: &GenerateFormData,
) -> web_sys::FormData {
    let form = web_sys::FormData::new().unwrap();
    let blob = build_metadata_blob(data);
    let _ = form.append_with_blob_and_filename(
        "metadata", &blob, "metadata.json",
    );
    for name in &data.files {
        let _ = form.append_with_str("files", name);
    }
    form
}

/// Serialize the form fields into a JSON metadata blob.
fn build_metadata_blob(
    data: &GenerateFormData,
) -> web_sys::Blob {
    let meta = serde_json::json!({
        "name": data.name,
        "description": data.description,
        "instructions": data.instructions,
        "language": data.language,
        "level": data.level,
        "num_questions": data.num_questions,
        "subjects": data.subjects,
        "output_game": "qcm",
    });
    let json_str: wasm_bindgen::JsValue =
        meta.to_string().into();
    let arr = js_sys::Array::of1(&json_str);
    web_sys::Blob::new_with_str_sequence(&arr)
        .unwrap()
}

/// Async API call triggered by the generate form.
#[derive(Clone)]
pub struct ApiCall(
    pub  Arc<
        dyn Fn(
            GenerateFormData,
        ) -> Pin<
            Box<dyn Future<Output = Result<(), String>>>,
        > + Send
            + Sync,
    >,
);

impl PartialEq for ApiCall {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Generic "generate a set" page for any game type.
#[component]
pub fn GameGeneratePage(
    /// Title shown in the hero banner.
    title: String,
    /// Toast message shown on success.
    success_msg: String,
    /// Path to navigate to after success.
    redirect: String,
    /// API call triggered on submit.
    api_call: ApiCall,
) -> Element {
    let _auth = use_auth_guard();
    let state = SubmitState {
        toast: use_toast(),
        loading: use_signal(|| false),
        err: use_signal(|| None),
    };
    let target = SubmitTarget { success_msg, redirect };
    let loading = state.loading;
    let err = state.err;
    let on_submit = submit_handler(api_call, target, state);

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                HeroBanner { title: title, span {} }
                GenerateForm {
                    on_submit: on_submit,
                    loading: loading,
                    error: err,
                }
            }
        }
    }
}

/// Success message + redirect path for the submit handler.
struct SubmitTarget {
    success_msg: String,
    redirect: String,
}

/// Reactive signals written by the submit handler.
#[derive(Clone, Copy)]
struct SubmitState {
    toast: crate::ui::ToastState,
    loading: Signal<bool>,
    err: Signal<Option<String>>,
}

/// Build the submit handler: fires `api` + updates state.
fn submit_handler(
    api: ApiCall,
    target: SubmitTarget,
    state: SubmitState,
) -> impl FnMut(GenerateFormData) + 'static {
    let nav = navigator();
    let SubmitState { mut toast, mut loading, mut err } = state;
    move |data: GenerateFormData| {
        loading.set(true);
        err.set(None);
        let fut = (api.0)(data);
        let msg = target.success_msg.clone();
        let path = target.redirect.clone();
        spawn(async move {
            match fut.await {
                Ok(()) => {
                    toast.success(msg);
                    nav.push(path);
                }
                Err(e) => err.set(Some(e)),
            }
            loading.set(false);
        });
    }
}
