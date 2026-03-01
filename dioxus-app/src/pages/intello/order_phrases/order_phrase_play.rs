//! Order phrase play page

use crate::api;
use crate::components::intello::order_phrase::order_phrase_player::OrderPhrasePlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Order phrase play page — receives id from route
pub fn OrderPhrasePlayPage(id: String) -> Element {
    let fetch: FetchSetById<_> =
        FetchSetById(Arc::new(|id: String| {
            Box::pin(async move {
                api::order_phrases::get_order_phrase_sets()
                    .await
                    .ok()
                    .and_then(|v| {
                        v.into_iter()
                            .find(|s| s.id == id)
                    })
            })
        }));

    let render: PlayerRenderer<_> =
        PlayerRenderer(Arc::new(|s, on_back| {
            rsx! {
                OrderPhrasePlayer {
                    set: s,
                    on_back: on_back,
                }
            }
        }));

    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/intello/order-phrases",
            fetch_set: fetch,
            render_player: render,
        }
    }
}
