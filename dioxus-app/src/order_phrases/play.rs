//! Order Phrases — play page
//! (route `/order-phrases/play/:id`).

use std::sync::Arc;

use dioxus::prelude::*;

use crate::game_engine::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};

use super::api;
use super::player::OrderPhrasePlayer;
use super::types::OrderPhraseSet;

/// Where to navigate when the user exits the player.
const BACK_PATH: &str = "/order-phrases";

/// Play an order phrase set identified by `id`.
pub fn OrderPhrasePlayPage(id: String) -> Element {
    let fetch_set = build_fetch_set();
    let render_player = build_render_player();

    rsx! {
        GamePlayPage {
            id: id,
            back_path: BACK_PATH,
            fetch_set: fetch_set,
            render_player: render_player,
        }
    }
}

/// Wrap `api::find_set_by_id` in the shared adapter.
fn build_fetch_set() -> FetchSetById<OrderPhraseSet> {
    FetchSetById(Arc::new(|id| {
        Box::pin(api::find_set_by_id(id))
    }))
}

/// Wrap the player component in the shared adapter.
fn build_render_player() -> PlayerRenderer<OrderPhraseSet> {
    PlayerRenderer(Arc::new(|set, on_back| {
        rsx! {
            OrderPhrasePlayer {
                set: set,
                on_back: on_back,
            }
        }
    }))
}
