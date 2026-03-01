// ** game_play_page.rs **
// ==> Shared play page: fetch set by id, show player

use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::nav_callback;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::state::auth::use_auth_guard;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Fetch a set by ID — returns Option<T>
pub type FetchSetById<T> = Arc<
    dyn Fn(String) -> Pin<
        Box<dyn Future<Output = Option<T>>>,
    > + Send
        + Sync,
>;

/// Render callback — receives set + on_back
pub type PlayerRenderer<T> = Arc<
    dyn Fn(T, Callback<()>) -> AnyView
        + Send
        + Sync,
>;

/// Shared play page for any game type
#[component]
pub fn GamePlayPage<T>(
    /// Path to navigate back to list
    #[prop(into)]
    back_path: String,
    /// Fetches the set by ID
    fetch_set: FetchSetById<T>,
    /// Renders the player component
    render_player: PlayerRenderer<T>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let _auth = use_auth_guard();
    let params = use_params_map();
    let set: RwSignal<Option<T>> =
        RwSignal::new(None);
    let loading = RwSignal::new(true);

    Effect::new({
        let mut fired = false;
        let fetch = fetch_set.clone();
        move |_| {
            let id = params
                .get()
                .get("id")
                .unwrap_or_default();
            if id.is_empty() || fired {
                return;
            }
            fired = true;
            let fut = fetch(id);
            spawn_local(async move {
                set.set(fut.await);
                loading.set(false);
            });
        }
    });

    let on_back = nav_callback(back_path);
    let renderer =
        StoredValue::new(render_player);

    view! {
        <HomeTopBar />
        <PageLayout>
            <Show
                when=move || !loading.get()
                fallback=|| view! { <Spinner /> }
            >
                {move || {
                    set.get().map(|s| {
                        renderer.with_value(|r| {
                            r(s, on_back)
                        })
                    })
                }}
            </Show>
        </PageLayout>
    }
}
