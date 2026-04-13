//! Displays generated course session content

use crate::api;
use crate::components::course::ContentRenderer;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::loading_boundary::LoadingBoundary;
use crate::components::ui::page_layout::PageLayout;
use crate::domain::course_types::SessionData;
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Session view page — receives course_id and sid
pub fn SessionViewPage(
    course_id: String,
    sid: String,
) -> Element {
    let _auth = use_auth_guard();

    let loading = use_signal(|| true);
    let session: Signal<Option<SessionData>> =
        use_signal(|| None);

    let cid = course_id.clone();
    let sid_c = sid.clone();
    let fetch: FetchFn<Option<SessionData>> =
        Box::new(move || {
            let cid = cid.clone();
            let sid = sid_c.clone();
            Box::pin(async move {
                api::courses::get_session(&cid, &sid)
                    .await
                    .map(Some)
            })
        });
    use_fetch_on_mount(session, loading, fetch);

    rsx! {
        HomeTopBar {}
        PageLayout {
            LoadingBoundary {
                loading: loading,
                if let Some(s) = (session)() {
                    if let Some(c) = s.content {
                        ContentRenderer {
                            course: c,
                        }
                    }
                }
            }
        }
    }
}
