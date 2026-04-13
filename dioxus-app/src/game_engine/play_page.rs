//! Generic "play a set" page: load by id, render player.

use crate::components::top_bar::HomeTopBar;
use crate::state::auth::use_auth_guard;
use crate::ui::{PageLayout, Spinner};
use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Fetches a set by ID asynchronously.
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

/// Renders the game-specific player for a loaded set.
#[derive(Clone)]
pub struct PlayerRenderer<T>(
    pub  Arc<
        dyn Fn(T, EventHandler<()>) -> Element + Send + Sync,
    >,
);

impl<T> PartialEq for PlayerRenderer<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Shared "play a set" page.
#[component]
pub fn GamePlayPage<
    T: Clone + PartialEq + Send + Sync + 'static,
>(
    /// ID of the set to load.
    id: String,
    /// Path used when the player signals "back".
    back_path: String,
    /// Loads the set by ID.
    fetch_set: FetchSetById<T>,
    /// Renders the player once the set is loaded.
    render_player: PlayerRenderer<T>,
) -> Element {
    let _auth = use_auth_guard();
    let set: Signal<Option<T>> = use_signal(|| None);
    let loading = use_signal(|| true);

    start_load(id.clone(), fetch_set.clone(), set, loading);
    let on_back = back_handler(back_path.clone());

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            PageLayout {
                if loading() {
                    Spinner {}
                } else if let Some(s) = set() {
                    {(render_player.0)(
                        s,
                        EventHandler::new(on_back),
                    )}
                }
            }
        }
    }
}

/// Kick off the fetch effect on mount.
fn start_load<T>(
    id: String,
    fetch: FetchSetById<T>,
    mut set: Signal<Option<T>>,
    mut loading: Signal<bool>,
) where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    use_effect(move || {
        let fetch = fetch.0.clone();
        let id = id.clone();
        spawn(async move {
            set.set(fetch(id).await);
            loading.set(false);
        });
    });
}

/// Build the "go back" handler that pushes `back_path`.
fn back_handler(back_path: String) -> impl FnMut(()) + 'static {
    let nav = navigator();
    move |_| {
        nav.push(back_path.clone());
    }
}
