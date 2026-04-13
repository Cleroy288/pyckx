//! True/False play page

use crate::api;
use crate::components::true_false::true_false_player::TrueFalsePlayer;
use crate::components::game_shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// True/False play page — receives id from route
pub fn TrueFalsePlayPage(id: String) -> Element {
    let fetch: FetchSetById<_> =
        FetchSetById(Arc::new(|id: String| {
            Box::pin(async move {
                api::true_false::get_true_false_sets()
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
                TrueFalsePlayer {
                    set: s,
                    on_back: on_back,
                }
            }
        }));

    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/true-false",
            fetch_set: fetch,
            render_player: render,
        }
    }
}
