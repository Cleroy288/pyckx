//! Shared list page for all game types

use crate::components::game_shared::{
    game_set_card::GameSetCard, set_list::SetList,
};
use crate::home::HomeTopBar;
use crate::ui::{
    Button, ButtonVariant,
};
use crate::ui::use_toast;
use crate::domain::game_set_trait::GameSetInfo;
use crate::state::auth::use_auth_guard;
use dioxus::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Async fetch signature for game sets
#[derive(Clone)]
pub struct FetchSets<T>(
    pub  Arc<
        dyn Fn() -> Pin<
            Box<
                dyn Future<
                    Output = Result<Vec<T>, String>,
                >,
            >,
        > + Send
            + Sync,
    >,
);

impl<T> PartialEq for FetchSets<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Async delete signature
#[derive(Clone)]
pub struct DeleteFn(
    pub  Arc<
        dyn Fn(String) -> Pin<
            Box<
                dyn Future<
                    Output = Result<bool, String>,
                >,
            >,
        > + Send
            + Sync,
    >,
);

impl PartialEq for DeleteFn {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// Shared list page for any game type
#[component]
pub fn GameListPage<
    T: GameSetInfo
        + Clone
        + PartialEq
        + Send
        + Sync
        + 'static,
>(
    /// Page title in hero banner
    title: String,
    /// Label for empty state
    game_label: String,
    /// Path to the generate page
    generate_path: String,
    /// Path prefix for play (+ /{id})
    play_path_prefix: String,
    /// Fetch all sets
    fetch_sets: FetchSets<T>,
    /// Optional create button path
    #[props(default)]
    create_path: Option<String>,
    /// Optional delete function
    #[props(default)]
    delete_fn: Option<DeleteFn>,
    /// Optional quick-play path
    #[props(default)]
    quick_path: Option<String>,
) -> Element {
    let _auth = use_auth_guard();
    let mut toast = use_toast();
    let mut sets: Signal<Vec<T>> =
        use_signal(Vec::new);
    let mut loading = use_signal(|| true);

    let has = use_memo(move || !(sets)().is_empty());

    // Fetch on mount
    use_effect({
        let fetch = fetch_sets.0.clone();
        move || {
            let fetch = fetch.clone();
            spawn(async move {
                match fetch().await {
                    Ok(data) => sets.set(data),
                    Err(e) => {
                        toast.error(e);
                    }
                }
                loading.set(false);
            });
        }
    });

    let nav = navigator();
    let gen_path = generate_path.clone();
    let play_prefix = play_path_prefix.clone();

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            main { class: "home-main",
                header { class: "gl-hero",
                    div { class: "gl-hero-stripes" }
                    div { class: "gl-hero-body",
                        span { class: "gl-eyebrow",
                            "\u{2726} Game Mode"
                        }
                        h1 { class: "gl-title",
                            "{title}"
                        }
                        div {
                            class: "gl-actions",
                            if let Some(ref cp) =
                                create_path
                            {
                                Button {
                                    text: "Create",
                                    on_click: {
                                        let c =
                                            cp.clone();
                                        move |_| {
                                            nav.push(
                                                c.clone(),
                                            );
                                        }
                                    },
                                }
                            }
                            Button {
                                text: "Generate",
                                variant:
                                    ButtonVariant::Outline,
                                on_click: {
                                    let g =
                                        gen_path.clone();
                                    move |_| {
                                        nav.push(
                                            g.clone(),
                                        );
                                    }
                                },
                            }
                            if let Some(ref qp) =
                                quick_path
                            {
                                Button {
                                    text: "Quick",
                                    variant:
                                        ButtonVariant::Outline,
                                    on_click: {
                                        let q =
                                            qp.clone();
                                        move |_| {
                                            nav.push(
                                                q.clone(),
                                            );
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
                div { class: "home-section",
                    SetList {
                        loading: loading,
                        has_items: has,
                        game_label: game_label,
                        {render_cards(
                            sets,
                            play_prefix,
                            delete_fn.clone(),
                            toast,
                        )}
                    }
                }
            }
        }
    }
}

/// Shared context for rendering game set cards
struct CardCtx<T: 'static> {
    /// All game sets signal
    sets: Signal<Vec<T>>,
    /// Path prefix for play route
    play_prefix: String,
    /// Optional delete function
    delete_fn: Option<DeleteFn>,
    /// Toast state for notifications
    toast: crate::ui::ToastState,
}

/// Render the list of game set cards
fn render_cards<T>(
    sets: Signal<Vec<T>>,
    play_prefix: String,
    delete_fn: Option<DeleteFn>,
    toast: crate::ui::ToastState,
) -> Element
where
    T: GameSetInfo + Send + Sync + 'static,
{
    let ctx = CardCtx {
        sets,
        play_prefix,
        delete_fn,
        toast,
    };
    rsx! {
        for set in (sets)().iter() {
            div { key: "{set.id()}",
                {render_single_card(set, &ctx)}
            }
        }
    }
}

/// Render a single GameSetCard
fn render_single_card<T>(
    set: &T,
    ctx: &CardCtx<T>,
) -> Element
where
    T: GameSetInfo + Send + Sync + 'static,
{
    let id = set.id().to_string();
    let path =
        format!("{}/{}", ctx.play_prefix, id);
    let del_fn = ctx.delete_fn.clone();
    let del_id = id.clone();
    let mut toast_c = ctx.toast;
    let mut sets = ctx.sets;
    let nav = navigator();

    rsx! {
        GameSetCard {
            name: set.name().to_string(),
            level: set.level_str(),
            subjects: set.subjects().to_vec(),
            count: set.item_count(),
            item_label: T::item_label(),
            on_play: move |_| {
                nav.push(path.clone());
            },
            on_delete: move |_| {
                let Some(df) = &del_fn else {
                    return;
                };
                let d = del_id.clone();
                let fut = (df.0)(d.clone());
                spawn(async move {
                    let Ok(true) = fut.await else {
                        return;
                    };
                    sets.write().retain(
                        |s| s.id() != d,
                    );
                    toast_c.success(
                        "Deleted".into(),
                    );
                });
            },
        }
    }
}
