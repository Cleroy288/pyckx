// ** game_generate_page.rs **
// ==> Shared page layout for all game generation pages

use crate::components::intello::shared::{
    generate_form::{GenerateForm, GenerateFormData},
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::toast::use_toast;
use crate::state::auth::use_auth_guard;
use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Async API call signature for game generation
#[derive(Clone)]
pub struct ApiCall(
    pub  Arc<
        dyn Fn(
            GenerateFormData,
        ) -> Pin<
            Box<
                dyn Future<
                    Output = Result<(), String>,
                >,
            >,
        > + Send
            + Sync,
    >,
);

impl PartialEq for ApiCall {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Builds web_sys::FormData from GenerateFormData
pub fn build_form_data(
    data: &GenerateFormData,
) -> web_sys::FormData {
    let form = web_sys::FormData::new().unwrap();
    let _ = form.append_with_str("name", &data.name);
    let _ = form.append_with_str(
        "description",
        &data.description,
    );
    let _ = form.append_with_str(
        "instructions",
        &data.instructions,
    );
    let _ = form.append_with_str(
        "language",
        &data.language,
    );
    let _ = form.append_with_str(
        "level",
        &data.level,
    );
    let _ = form.append_with_str(
        "num_questions",
        &data.num_questions.to_string(),
    );
    for s in &data.subjects {
        let _ = form.append_with_str("subjects", s);
    }
    // TODO: File upload needs rework for Dioxus —
    // FileEngine gives names, not File objects.
    for name in &data.files {
        let _ =
            form.append_with_str("files", name);
    }
    form
}

/// Shared page for all game generation
#[component]
pub fn GameGeneratePage(
    /// Page title displayed in hero banner
    title: String,
    /// Toast message on success
    success_msg: String,
    /// Path to navigate after success
    redirect: String,
    /// Async API call
    api_call: ApiCall,
) -> Element {
    let _auth = use_auth_guard();
    let mut toast = use_toast();
    let mut loading = use_signal(|| false);
    let mut err: Signal<Option<String>> =
        use_signal(|| None);
    let nav = navigator();

    let on_submit =
        move |data: GenerateFormData| {
            loading.set(true);
            err.set(None);
            let fut = (api_call.0)(data);
            let msg = success_msg.clone();
            let path = redirect.clone();
            let nav = nav.clone();
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
        };

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner { title: title,
                span {}
            }
            GenerateForm {
                on_submit: on_submit,
                loading: loading,
                error: err,
            }
        }
    }
}
