// ** game_list_page.rs **
// ==> Shared list page for all game types

use crate::components::intello::shared::{
    game_set_card::GameSetCard, set_list::SetList,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{
    Button, ButtonVariant,
};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::toast::use_toast;
use crate::domain::game_set_trait::GameSetInfo;
use crate::state::auth::use_auth_guard;
use dioxus::prelude::*;
use dioxus_router::Navigator;
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
        HomeTopBar {}
        PageLayout {
            HeroBanner { title: title,
                div {
                    class: "absolute bottom-3 \
                        left-4 z-[1] flex gap-2",
                    if let Some(cp) = &create_path {
                        Button {
                            text: "Create",
                            on_click: {
                                let nav = nav.clone();
                                let cp = cp.clone();
                                move |_| {
                                    nav.push(cp.clone());
                                }
                            },
                        }
                    }
                    Button {
                        text: "Generate",
                        variant: ButtonVariant::Outline,
                        on_click: {
                            let nav = nav.clone();
                            let gp = gen_path.clone();
                            move |_| {
                                nav.push(gp.clone());
                            }
                        },
                    }
                    if let Some(qp) = &quick_path {
                        Button {
                            text: "Quick",
                            variant: ButtonVariant::Outline,
                            on_click: {
                                let nav = nav.clone();
                                let qp = qp.clone();
                                move |_| {
                                    nav.push(qp.clone());
                                }
                            },
                        }
                    }
                }
            }
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

/// Render the list of game set cards
fn render_cards<T>(
    sets: Signal<Vec<T>>,
    play_prefix: String,
    delete_fn: Option<DeleteFn>,
    mut toast: crate::components::ui::toast::ToastState,
) -> Element
where
    T: GameSetInfo + Send + Sync + 'static,
{
    let nav = navigator();
    rsx! {
        for set in (sets)().iter() {
            {render_single_card(
                set,
                &play_prefix,
                &delete_fn,
                sets,
                &mut toast,
                &nav,
            )}
        }
    }
}

/// Render a single GameSetCard
fn render_single_card<T>(
    set: &T,
    play_prefix: &str,
    delete_fn: &Option<DeleteFn>,
    mut sets: Signal<Vec<T>>,
    toast: &mut crate::components::ui::toast::ToastState,
    nav: &Navigator,
) -> Element
where
    T: GameSetInfo + Send + Sync + 'static,
{
    let id = set.id().to_string();
    let path = format!("{}/{}", play_prefix, id);
    let del_fn = delete_fn.clone();
    let del_id = id.clone();
    let mut toast_c = *toast;
    let nav = nav.clone();

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
