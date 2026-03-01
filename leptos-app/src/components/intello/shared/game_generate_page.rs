// ** game_generate_page.rs **
// ==> Shared page layout for all game generation pages

use crate::components::intello::shared::generate_form::{
    GenerateForm, GenerateFormData,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::toast::use_toast;
use crate::state::auth::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Async API call signature for game generation
pub type ApiCall = Arc<
    dyn Fn(
        GenerateFormData,
    ) -> Pin<
        Box<dyn Future<Output = Result<(), String>>>,
    > + Send
        + Sync,
>;

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
    let _ =
        form.append_with_str("language", &data.language);
    let _ =
        form.append_with_str("level", &data.level);
    let _ = form.append_with_str(
        "num_questions",
        &data.num_questions.to_string(),
    );
    for s in &data.subjects {
        let _ = form.append_with_str("subjects", s);
    }
    for f in &data.files {
        let _ = form.append_with_blob("files", f);
    }
    form
}

/// Shared page for all game generation
#[component]
pub fn GameGeneratePage(
    /// Page title displayed in hero banner
    #[prop(into)]
    title: String,
    /// Toast message on success
    #[prop(into)]
    success_msg: String,
    /// Path to navigate after success
    #[prop(into)]
    redirect: String,
    /// Async API call
    api_call: ApiCall,
) -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();
    let loading = RwSignal::new(false);
    let err: RwSignal<Option<String>> =
        RwSignal::new(None);
    let nav = use_navigate();

    let on_submit = Callback::new(
        move |data: GenerateFormData| {
            loading.set(true);
            err.set(None);
            let fut = (api_call)(data);
            let msg = success_msg.clone();
            let path = redirect.clone();
            let nav = nav.clone();
            spawn_local(async move {
                match fut.await {
                    Ok(()) => {
                        toast.success(msg);
                        nav(
                            &path,
                            Default::default(),
                        );
                    }
                    Err(e) => err.set(Some(e)),
                }
                loading.set(false);
            });
        },
    );

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title=title>
                <span />
            </HeroBanner>
            <GenerateForm
                on_submit=on_submit
                loading=loading
                error=err
            />
        </PageLayout>
    }
}
