// ** game_play_page.rs **
// ==> Shared play page: fetch set by id, show player

use crate::components::top_bar::HomeTopBar;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::spinner::Spinner;
use crate::state::auth::use_auth_guard;
use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Fetch a set by ID — returns Option<T>
#[derive(Clone)]
pub struct FetchSetById<T>(
    pub  Arc<
        dyn Fn(String) -> Pin<
            Box<dyn Future<Output = Option<T>>>,
        > + Send
            + Sync,
    >,
);

impl<T> PartialEq for FetchSetById<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Render callback — receives set + on_back
#[derive(Clone)]
pub struct PlayerRenderer<T>(
    pub  Arc<
        dyn Fn(T, EventHandler<()>) -> Element
            + Send
            + Sync,
    >,
);

impl<T> PartialEq for PlayerRenderer<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Shared play page for any game type
#[component]
pub fn GamePlayPage<
    T: Clone + PartialEq + Send + Sync + 'static,
>(
    /// The set ID to fetch
    id: String,
    /// Path to navigate back to list
    back_path: String,
    /// Fetches the set by ID
    fetch_set: FetchSetById<T>,
    /// Renders the player component
    render_player: PlayerRenderer<T>,
) -> Element {
    let _auth = use_auth_guard();
    let mut set: Signal<Option<T>> =
        use_signal(|| None);
    let mut loading = use_signal(|| true);

    use_effect({
        let fetch = fetch_set.0.clone();
        let id = id.clone();
        move || {
            let fetch = fetch.clone();
            let id = id.clone();
            spawn(async move {
                set.set(fetch(id).await);
                loading.set(false);
            });
        }
    });

    let nav = navigator();
    let bp = back_path.clone();
    let on_back = move |_: ()| {
        nav.push(bp.clone());
    };

    rsx! {
        HomeTopBar {}
        PageLayout {
            if (loading)() {
                Spinner {}
            } else if let Some(s) = (set)() {
                {(render_player.0)(
                    s,
                    EventHandler::new(on_back),
                )}
            }
        }
    }
}
