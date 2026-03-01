use crate::api;
use crate::components::intello::order_phrase::order_phrase_player::OrderPhrasePlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Order phrase play page
#[component]
pub fn OrderPhrasePlayPage() -> impl IntoView {
    let fetch: FetchSetById<_> =
        Arc::new(|id: String| {
            Box::pin(async move {
                api::order_phrases::get_order_phrase_sets()
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
                <OrderPhrasePlayer
                    set=s on_back=on_back
                />
            }
            .into_any()
        });

    view! {
        <GamePlayPage
            back_path="/intello/order-phrases"
            fetch_set=fetch
            render_player=render
        />
    }
}
