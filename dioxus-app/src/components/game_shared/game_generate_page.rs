//! Shared page layout for all game generation pages

use crate::components::game_shared::{
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

/// Re-export build_form_data from api layer
pub use crate::api::form_data::build_generate_form
    as build_form_data;

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
        div { class: "home-page",
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
}
