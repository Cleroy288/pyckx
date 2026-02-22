use crate::api;
use crate::components::intello::fill_blank::fill_blank_player::FillBlankPlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Fill blank play page
#[component]
pub fn FillBlankPlayPage() -> impl IntoView {
    let fetch: FetchSetById<_> =
        Arc::new(|id: String| {
            Box::pin(async move {
                api::fill_blanks::get_fill_blank_sets()
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
                <FillBlankPlayer
                    set=s on_back=on_back
                />
            }
            .into_any()
        });

    view! {
        <GamePlayPage
            back_path="/intello/fill-blanks"
            fetch_set=fetch
            render_player=render
        />
    }
}
