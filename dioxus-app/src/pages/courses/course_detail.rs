//! Course detail page with resources and sessions

use crate::api;
use crate::components::course::{
    ResourceList, SessionCard,
};
use crate::home::HomeTopBar;
use crate::ui::Button;
use crate::ui::HeroBanner;
use crate::ui::LoadingBoundary;
use crate::ui::PageLayout;
use crate::ui::{TabPanel, Tabs};
use crate::domain::course_types::{
    ResourceData, SessionData,
};
use crate::state::use_auth_guard;
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

/// Course detail page — tabs: Resources + Sessions
pub fn CourseDetailPage(id: String) -> Element {
    let _auth = use_auth_guard();
    let nav = navigator();

    let mut tab = use_signal(|| 0_usize);
    let mut loading = use_signal(|| true);
    let mut resources: Signal<Vec<ResourceData>> =
        use_signal(Vec::new);
    let mut sessions: Signal<Vec<SessionData>> =
        use_signal(Vec::new);

    // Load resources + sessions once on mount
    let cid = id.clone();
    use_effect(move || {
        let cid = cid.clone();
        spawn(async move {
            let (res, sess) = (
                api::courses::get_resources(&cid)
                    .await,
                api::courses::list_sessions(&cid)
                    .await,
            );
            if let Ok(r) = res {
                resources.set(r);
            }
            if let Ok(s) = sess {
                sessions.set(s);
            }
            loading.set(false);
        });
    });

    let cid_for_upload = id.clone();
    let on_upload = move |_: ()| {
        let cid = cid_for_upload.clone();
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
                );
            });
        html_input.set_onchange(Some(
            closure.as_ref().unchecked_ref(),
        ));
        closure.forget();
        html_input.click();
    };

    let tab_labels =
        vec!["Resources".into(), "Sessions".into()];

    let cid_for_nav = id.clone();
    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "Course Detail",
                div {
                    class: "absolute bottom-3 left-4 \
                        z-[1] flex gap-2",
                    Button {
                        text: "New Session".to_string(),
                        on_click: {
                            let cid = cid_for_nav.clone();
                            move |_| {
                                nav.push(format!(
                                    "/courses/{}/session/create",
                                    cid,
                                ));
                            }
                        },
                    }
                }
            }
            LoadingBoundary {
                loading: loading,
                Tabs {
                    labels: tab_labels.clone(),
                    active: tab,
                    on_change: move |i: usize| {
                        tab.set(i);
                    },
                }
                TabPanel {
                    index: 0,
                    active: tab,
                    ResourceList {
                        resources: resources,
                        on_upload: on_upload,
                    }
                }
                TabPanel {
                    index: 1,
                    active: tab,
                    div {
                        class: "flex flex-col gap-4",
                        for sess in (sessions)().iter() {
                            {
                                let sid = sess.id.clone();
                                let path = format!(
                                    "/courses/{}/session/{}",
                                    id,
                                    sess.id,
                                );
                                rsx! {
                                    SessionCard {
                                        key: "{sid}",
                                        topic: sess.topic.clone(),
                                        status: sess.status.clone(),
                                        created_at: sess.created_at.clone(),
                                        on_view: move |_| {
                                            nav.push(&*path);
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Read file from input and upload to server
fn upload_selected_file(
    input: &web_sys::HtmlInputElement,
    cid: String,
    mut resources: Signal<Vec<ResourceData>>,
) {
    let Some(fl) = input.files() else { return };
    let Some(file) = fl.get(0) else { return };
    let form = web_sys::FormData::new().unwrap();
    let _ = form.append_with_blob("file", &file);
    spawn(async move {
        let result =
            api::courses::upload_resource(
                &cid, &form,
            )
            .await;
        if let Ok(r) = result {
            resources.write().push(r);
        }
    });
}
