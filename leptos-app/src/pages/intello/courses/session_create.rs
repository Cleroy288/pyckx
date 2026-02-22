// ** session_create.rs **
// ==> Create a study session with topic and resources

use crate::api;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{btn_click, Button};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::input::Input;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::components::ui::textarea::Textarea;
use crate::components::ui::toast::use_toast;
use crate::domain::course_types::CreateSessionRequest;
use crate::state::hooks::use_async_navigate;
use crate::state::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;

stylance::import_crate_style!(
    style,
    "src/pages/intello/courses/session_create.module.css"
);

/// Create session page
#[component]
pub fn SessionCreatePage() -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();
    let nav_to = use_async_navigate();
    let params = use_params_map();

    let course_id = Signal::derive(move || {
        params.get().get("id").unwrap_or_default()
    });

    let topic = RwSignal::new(String::new());
    let keywords = RwSignal::new(String::new());
    let instr = RwSignal::new(String::new());
    let lang = RwSignal::new("en".to_string());
    let loading = RwSignal::new(false);

    let valid = Signal::derive(move || {
        !topic.get().trim().is_empty()
    });

    let handle = move |_| {
        loading.set(true);
        let cid = course_id.get();
        spawn_local(async move {
            let kw: Vec<String> = keywords
                .get()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            let req = CreateSessionRequest {
                topic: topic.get(),
                keywords: kw,
                instructions: instr.get(),
                language: lang.get(),
                resource_ids: Vec::new(),
            };
            match api::courses::create_session(
                &cid, &req,
            )
            .await
            {
                Ok(sess) => {
                    toast.success(
                        "Session created".into(),
                    );
                    nav_to.set(Some(format!(
                        "/intello/courses/{}/session/{}",
                        cid, sess.id
                    )));
                }
                Err(e) => {
                    toast.error(e);
                    loading.set(false);
                }
            }
        });
    };

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="New Session">
                <span />
            </HeroBanner>
            <div class=style::form>
                <Input
                    type_="text"
                    id="sess-topic"
                    label="Topic".to_string()
                    placeholder="Session topic"
                        .to_string()
                    value=topic
                    on_input=Callback::new(
                        move |v| topic.set(v),
                    )
                    required=true
                />
                <Input
                    type_="text"
                    id="sess-kw"
                    label="Keywords (comma separated)"
                        .to_string()
                    placeholder="AI, machine learning"
                        .to_string()
                    value=keywords
                    on_input=Callback::new(
                        move |v| keywords.set(v),
                    )
                />
                <Textarea
                    id="sess-instr"
                    label="Instructions".to_string()
                    placeholder="Additional instructions..."
                        .to_string()
                    value=instr
                    on_input=Callback::new(
                        move |v| instr.set(v),
                    )
                />
                <Input
                    type_="text"
                    id="sess-lang"
                    label="Language".to_string()
                    value=lang
                    on_input=Callback::new(
                        move |v| lang.set(v),
                    )
                />
                <Show
                    when=move || loading.get()
                    fallback=move || {
                        view! {
                            <Button
                                text="Create Session"
                                disabled=Signal::derive(
                                    move || !valid.get(),
                                )
                                on_click=btn_click(
                                    handle,
                                )
                            />
                        }
                    }
                >
                    <Spinner />
                </Show>
            </div>
        </PageLayout>
    }
}
