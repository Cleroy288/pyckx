//! Generic list page showing all sets of a game type.

use crate::components::top_bar::HomeTopBar;
use crate::game_engine::set_card::GameSetCard;
use crate::game_engine::set_list::SetList;
use crate::game_engine::traits::GameSetInfo;
use crate::state::auth::use_auth_guard;
use crate::ui::{use_toast, Button, ButtonVariant, ToastState};
use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Async fetch for a list of game sets.
#[derive(Clone)]
pub struct FetchSets<T>(
    pub  Arc<
        dyn Fn() -> Pin<
            Box<dyn Future<Output = Result<Vec<T>, String>>>,
        > + Send
            + Sync,
    >,
);

impl<T> PartialEq for FetchSets<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Async deletion by ID.
#[derive(Clone)]
pub struct DeleteFn(
    pub  Arc<
        dyn Fn(String) -> Pin<
            Box<dyn Future<Output = Result<bool, String>>>,
        > + Send
            + Sync,
    >,
);

impl PartialEq for DeleteFn {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Generic list page for any game type.
#[component]
pub fn GameListPage<
    T: GameSetInfo + Clone + PartialEq + Send + Sync + 'static,
>(
    /// Title shown in the hero banner.
    title: String,
    /// Label used in the empty state.
    game_label: String,
    /// Path to the "generate" page.
    generate_path: String,
    /// Path prefix for play (set ID is appended).
    play_path_prefix: String,
    /// Fetches all sets on mount.
    fetch_sets: FetchSets<T>,
    /// Optional path to the "create" page.
    #[props(default)]
    create_path: Option<String>,
    /// Optional delete callback.
    #[props(default)]
    delete_fn: Option<DeleteFn>,
    /// Optional quick-play path.
    #[props(default)]
    quick_path: Option<String>,
) -> Element {
    let _auth = use_auth_guard();
    let toast = use_toast();
    let sets: Signal<Vec<T>> = use_signal(Vec::new);
    let loading = use_signal(|| true);
    let has = use_memo(move || !sets().is_empty());

    start_fetch(fetch_sets.clone(), sets, loading, toast);

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            main { class: "home-main",
                {hero_banner(
                    title,
                    create_path,
                    generate_path,
                    quick_path,
                )}
                div { class: "home-section",
                    SetList {
                        loading: loading,
                        has_items: has,
                        game_label: game_label,
                        for set in sets().iter() {
                            div { key: "{set.id()}",
                                {render_card(
                                    set,
                                    &play_path_prefix,
                                    delete_fn.clone(),
                                    sets,
                                    toast,
                                )}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Render the hero banner with title + nav buttons.
fn hero_banner(
    title: String,
    create_path: Option<String>,
    generate_path: String,
    quick_path: Option<String>,
) -> Element {
    let nav = navigator();
    rsx! {
        header { class: "gl-hero",
            div { class: "gl-hero-stripes" }
            div { class: "gl-hero-body",
                span { class: "gl-eyebrow", "\u{2726} Game Mode" }
                h1 { class: "gl-title", "{title}" }
                div { class: "gl-actions",
                    if let Some(path) = create_path {
                        Button {
                            text: "Create",
                            on_click: move |_| { nav.push(path.clone()); },
                        }
                    }
                    Button {
                        text: "Generate",
                        variant: ButtonVariant::Outline,
                        on_click: move |_| {
                            nav.push(generate_path.clone());
                        },
                    }
                    if let Some(path) = quick_path {
                        Button {
                            text: "Quick",
                            variant: ButtonVariant::Outline,
                            on_click: move |_| { nav.push(path.clone()); },
                        }
                    }
                }
            }
        }
    }
}

/// Start the fetch effect and populate `sets`.
fn start_fetch<T>(
    fetch: FetchSets<T>,
    mut sets: Signal<Vec<T>>,
    mut loading: Signal<bool>,
    mut toast: ToastState,
) where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    use_effect(move || {
        let fetch = fetch.0.clone();
        spawn(async move {
            match fetch().await {
                Ok(data) => sets.set(data),
                Err(e) => toast.error(e),
            }
            loading.set(false);
        });
    });
}

/// Render a single `GameSetCard` with its callbacks wired.
fn render_card<T>(
    set: &T,
    play_prefix: &str,
    delete_fn: Option<DeleteFn>,
    mut sets: Signal<Vec<T>>,
    mut toast: ToastState,
) -> Element
where
    T: GameSetInfo + Send + Sync + 'static,
{
    let id = set.id().to_string();
    let path = format!("{}/{}", play_prefix, id);
    let nav = navigator();
    let del_id = id.clone();

    rsx! {
        GameSetCard {
            name: set.name().to_string(),
            level: set.level_str(),
            subjects: set.subjects().to_vec(),
            count: set.item_count(),
            item_label: T::item_label(),
            on_play: move |_| { nav.push(path.clone()); },
            on_delete: move |_| {
                let Some(df) = delete_fn.clone() else { return };
                let d = del_id.clone();
                spawn(async move {
                    let Ok(true) = (df.0)(d.clone()).await else {
                        return;
                    };
                    sets.write().retain(|s| s.id() != d);
                    toast.success("Deleted".into());
                });
            },
        }
    }
}
