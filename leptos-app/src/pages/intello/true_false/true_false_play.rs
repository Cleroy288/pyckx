use crate::api;
use crate::components::intello::true_false::true_false_player::TrueFalsePlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use leptos::prelude::*;
use std::sync::Arc;

/// True/False play page
#[component]
pub fn TrueFalsePlayPage() -> impl IntoView {
    let fetch: FetchSetById<_> =
        Arc::new(|id: String| {
            Box::pin(async move {
                api::true_false::get_true_false_sets()
                    .await
                    .ok()
                    .and_then(|v| {
                        v.into_iter()
                            .find(|s| s.id == id)
                    })
            }) as _
        });

    let render: PlayerRenderer<_> =
        Arc::new(|s, on_back| {
            view! {
                <TrueFalsePlayer
                    set=s on_back=on_back
                />
            }
            .into_any()
        });

    view! {
        <GamePlayPage
            back_path="/intello/true-false"
            fetch_set=fetch
            render_player=render
        />
    }
}
