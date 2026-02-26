//! Fill blank play page

use crate::api;
use crate::components::intello::fill_blank::fill_blank_player::FillBlankPlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Fill blank play page — receives id from route
pub fn FillBlankPlayPage(id: String) -> Element {
    let fetch: FetchSetById<_> =
        FetchSetById(Arc::new(|id: String| {
            Box::pin(async move {
                api::fill_blanks::get_fill_blank_sets()
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
                FillBlankPlayer {
                    set: s,
                    on_back: on_back,
                }
            }
        }));

    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/intello/fill-blanks",
            fetch_set: fetch,
            render_player: render,
        }
    }
}
