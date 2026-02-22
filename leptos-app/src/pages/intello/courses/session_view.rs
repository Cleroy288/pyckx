// ** session_view.rs **
// ==> Displays generated course session content

use crate::api;
use crate::components::intello::course::ContentRenderer;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::loading_boundary::LoadingBoundary;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::toast::use_toast;
use crate::domain::course_types::SessionData;
use crate::state::hooks::{use_fetch_on_mount, FetchFn};
use crate::state::use_auth_guard;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// Session view page — renders generated content
#[component]
pub fn SessionViewPage() -> impl IntoView {
    let _auth = use_auth_guard();
    let toast = use_toast();
    let params = use_params_map();

    let course_id = Signal::derive(move || {
        params.get().get("id").unwrap_or_default()
    });
    let session_id = Signal::derive(move || {
        params.get().get("sid").unwrap_or_default()
    });

    let loading = RwSignal::new(true);
    let session: RwSignal<Option<SessionData>> =
        RwSignal::new(None);

    // Load session data once on mount
    let fetch: FetchFn<Option<SessionData>> =
        Box::new(move || {
            let cid = course_id.get();
            let sid = session_id.get();
            Box::pin(async move {
                api::courses::get_session(&cid, &sid)
                    .await
                    .map(Some)
            })
        });
    use_fetch_on_mount(
        session, loading, toast, fetch,
    );

    view! {
        <HomeTopBar />
        <PageLayout>
            <LoadingBoundary loading=loading>
                {move || {
                    session.get().map(|s| {
                        view! {
                            {s.content.map(|c| {
                                view! {
                                    <ContentRenderer
                                        course=c
                                    />
                                }
                            })}
                        }
                    })
                }}
            </LoadingBoundary>
        </PageLayout>
    }
}
