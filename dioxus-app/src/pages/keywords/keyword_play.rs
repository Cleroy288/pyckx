//! Keywords play page

use crate::api;
use crate::components::keywords::keywords_player::KeywordsPlayer;
use crate::components::game_shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Keywords play page — receives id from route
pub fn KeywordsPlayPage(id: String) -> Element {
    let fetch: FetchSetById<_> =
        FetchSetById(Arc::new(|id: String| {
            Box::pin(async move {
                api::keywords::get_keyword_sets()
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
                KeywordsPlayer {
                    set: s,
                    on_back: on_back,
                }
            }
        }));

    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/keywords",
            fetch_set: fetch,
            render_player: render,
        }
    }
}
