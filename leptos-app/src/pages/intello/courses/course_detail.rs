// ** course_detail.rs **
// ==> Course detail page with resources and sessions

use crate::api;
use crate::components::intello::course::{
    ResourceList, SessionCard,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{
    nav_callback, nav_click, Button,
};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::loading_boundary::LoadingBoundary;
use crate::components::ui::tabs::{TabPanel, Tabs};
use crate::components::ui::toast::use_toast;
use crate::domain::course_types::{
    ResourceData, SessionData,
};
use crate::state::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use wasm_bindgen::JsCast;

stylance::import_crate_style!(
    style,
    "src/pages/intello/courses/course_detail.module.css"
);

/// Course detail page — tabs: Resources + Sessions
#[component]
pub fn CourseDetailPage() -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();
    let params = use_params_map();

    let id = Signal::derive(move || {
        params.get().get("id").unwrap_or_default()
    });

    let tab = RwSignal::new(0usize);
    let loading = RwSignal::new(true);
    let resources: RwSignal<Vec<ResourceData>> =
        RwSignal::new(Vec::new());
    let sessions: RwSignal<Vec<SessionData>> =
        RwSignal::new(Vec::new());

    // Load resources + sessions once on mount
    Effect::new({
        let mut fired = false;
        move |_| {
            let cid = id.get();
            if cid.is_empty() || fired {
                return;
            }
            fired = true;
            spawn_local(async move {
                let (res, sess) = (
                    api::courses::get_resources(&cid)
                        .await,
                    api::courses::list_sessions(&cid)
                        .await,
                );
                let _ =
                    res.map(|r| resources.set(r));
                let _ =
                    sess.map(|s| sessions.set(s));
                loading.set(false);
            });
        }
    });

    let on_upload =
        build_upload_cb(id, resources, toast);

    let tab_labels =
        vec!["Resources".into(), "Sessions".into()];

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="Course Detail">
                <div class=style::actions>
                    <Button
                        text="New Session"
                        on_click=nav_click(format!(
                            "/intello/courses/{}/session/create",
                            id.get()
                        ))
                    />
                </div>
            </HeroBanner>
            <LoadingBoundary loading=loading>
                <Tabs
                    labels=tab_labels.clone()
                    active=tab
                    on_change=Callback::new(
                        move |i| tab.set(i),
                    )
                />
                <TabPanel index=0 active=tab>
                    <ResourceList
                        resources=resources
                        on_upload=on_upload
                    />
                </TabPanel>
                <TabPanel index=1 active=tab>
                    <div class=style::sessions>
                        <For
                            each=move || {
                                sessions
                                    .with(Vec::clone)
                            }
                            key=|s| s.id.clone()
                            let:sess
                        >
                            {
                                let path = format!(
                                    "/intello/courses/{}/session/{}",
                                    id.get(),
                                    sess.id,
                                );
                                view! {
                                    <SessionCard
                                        topic=sess.topic.clone()
                                        status=sess.status.clone()
                                        created_at=sess.created_at.clone()
                                        on_view=nav_callback(path)
                                    />
                                }
                            }
                        </For>
                    </div>
                </TabPanel>
            </LoadingBoundary>
        </PageLayout>
    }
}

/// Builds the file upload callback
fn build_upload_cb(
    id: Signal<String>,
    resources: RwSignal<Vec<ResourceData>>,
    toast: crate::components::ui::toast::ToastState,
) -> Callback<()> {
    Callback::new(move |_: ()| {
        let cid = id.get();
        let input = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| {
                d.create_element("input").ok()
            });
        let Some(el) = input else { return };
        let _ = el.set_attribute("type", "file");
        let _ = el.set_attribute(
            "accept",
            ".pdf,.txt,.doc,.docx",
        );
        let html_input: web_sys::HtmlInputElement =
            el.unchecked_into();
        let html_clone = html_input.clone();
        let closure =
            wasm_bindgen::closure::Closure::<
                dyn Fn(),
            >::new(move || {
                upload_selected_file(
                    &html_clone,
                    cid.clone(),
                    resources,
                    toast,
                );
            });
        html_input.set_onchange(Some(
            closure.as_ref().unchecked_ref(),
        ));
        closure.forget();
        html_input.click();
    })
}

/// Read file from input and upload to server
fn upload_selected_file(
    input: &web_sys::HtmlInputElement,
    cid: String,
    resources: RwSignal<Vec<ResourceData>>,
    toast: crate::components::ui::toast::ToastState,
) {
    let Some(fl) = input.files() else { return };
    let Some(file) = fl.get(0) else { return };
    let form = web_sys::FormData::new().unwrap();
    let _ = form.append_with_blob("file", &file);
    spawn_local(async move {
        let result =
            api::courses::upload_resource(
                &cid, &form,
            )
            .await;
        match result {
            Ok(r) => {
                resources.update(|v| v.push(r));
                toast.success(
                    "Resource uploaded".into(),
                );
            }
            Err(e) => toast.error(e),
        }
    });
}
