// ** game_list_page.rs **
// ==> Shared list page for all game types

use crate::components::intello::shared::{
    game_set_card::GameSetCard, set_list::SetList,
};
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::button::{
    nav_callback, nav_click, Button, ButtonVariant,
};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use crate::components::ui::toast::use_toast;
use crate::domain::game_set_trait::GameSetInfo;
use crate::state::auth::use_auth_guard;
use crate::state::hooks::{
    has_items, use_fetch_on_mount, FetchFn,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

stylance::import_crate_style!(
    style,
    "src/components/intello/shared/\
     game_list_page.module.css"
);

/// Async fetch signature for game sets
pub type FetchSets<T> = Arc<
    dyn Fn() -> Pin<
        Box<
            dyn Future<
                Output = Result<Vec<T>, String>,
            >,
        >,
    > + Send
        + Sync,
>;

/// Async delete signature
pub type DeleteFn = Arc<
    dyn Fn(String) -> Pin<
        Box<
            dyn Future<
                Output = Result<bool, String>,
            >,
        >,
    > + Send
        + Sync,
>;

/// Shared list page for any game type
#[component]
pub fn GameListPage<T>(
    /// Page title in hero banner
    #[prop(into)]
    title: String,
    /// Label for empty state
    #[prop(into)]
    game_label: String,
    /// Path to the generate page
    #[prop(into)]
    generate_path: String,
    /// Path prefix for play (+ /{id})
    #[prop(into)]
    play_path_prefix: String,
    /// Fetch all sets
    fetch_sets: FetchSets<T>,
    /// Optional create button path
    #[prop(optional, into)]
    create_path: Option<String>,
    /// Optional delete function
    #[prop(optional)]
    delete_fn: Option<DeleteFn>,
    /// Optional quick-play path (renders extra button)
    #[prop(optional, into)]
    quick_path: Option<String>,
) -> impl IntoView
where
    T: GameSetInfo + Send + Sync + 'static,
{
    let _auth = use_auth_guard();
    let toast = use_toast();
    let sets: RwSignal<Vec<T>> =
        RwSignal::new(Vec::new());
    let loading = RwSignal::new(true);

    let fetch = fetch_sets.clone();
    let fetch_fn: FetchFn<Vec<T>> =
        Box::new(move || fetch());
    use_fetch_on_mount(sets, loading, toast, fetch_fn);

    let has = has_items(sets);

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title=title>
                <div class=style::actions>
                    {create_path.map(|cp| view! {
                        <Button
                            text="Create"
                            on_click=nav_click(cp)
                        />
                    })}
                    <Button
                        text="Generate"
                        variant=ButtonVariant::Outline
                        on_click=nav_click(generate_path)
                    />
                    {quick_path.map(|qp| view! {
                        <Button
                            text="Quick"
                            variant=ButtonVariant::Outline
                            on_click=nav_click(qp)
                        />
                    })}
                </div>
            </HeroBanner>
            <SetList
                loading=loading
                has_items=has
                game_label=game_label
            >
                {
                    let ctx = CardCtx {
                        play_prefix: play_path_prefix,
                        delete_fn,
                        sets,
                        toast,
                    };
                    view! {
                        <For
                            each=move || {
                                sets.with(Vec::clone)
                            }
                            key=|s| s.id().to_string()
                            let:set
                        >
                            {render_card(set, &ctx)}
                        </For>
                    }
                }
            </SetList>
        </PageLayout>
    }
}

/// Context for rendering a single card
#[derive(Clone)]
struct CardCtx<T: GameSetInfo + 'static> {
    play_prefix: String,
    delete_fn: Option<DeleteFn>,
    sets: RwSignal<Vec<T>>,
    toast: crate::components::ui::toast::ToastState,
}

/// Render a single GameSetCard
fn render_card<T>(
    set: T,
    ctx: &CardCtx<T>,
) -> impl IntoView
where
    T: GameSetInfo + Send + Sync + 'static,
{
    let id = set.id().to_string();
    let path =
        format!("{}/{}", ctx.play_prefix, id);
    let del_fn = ctx.delete_fn.clone();
    let sets = ctx.sets;
    let toast = ctx.toast;

    let on_delete = build_delete_cb(
        id.clone(), del_fn, sets, toast,
    );

    view! {
        <GameSetCard
            name=set.name().to_string()
            level=set.level_str()
            subjects=set.subjects().to_vec()
            count=set.item_count()
            item_label=T::item_label()
            on_play=nav_callback(path)
            on_delete=on_delete
        />
    }
}

/// Build the delete callback for a card
fn build_delete_cb<T>(
    id: String,
    delete_fn: Option<DeleteFn>,
    sets: RwSignal<Vec<T>>,
    toast: crate::components::ui::toast::ToastState,
) -> Callback<()>
where
    T: GameSetInfo + Send + Sync + 'static,
{
    Callback::new(move |_| {
        let Some(df) = &delete_fn else { return };
        let d = id.clone();
        let fut = df(d.clone());
        spawn_local(async move {
            let Ok(true) = fut.await else {
                return;
            };
            sets.update(
                |v| v.retain(|s| s.id() != d),
            );
            toast.success("Deleted".into());
        });
    })
}
