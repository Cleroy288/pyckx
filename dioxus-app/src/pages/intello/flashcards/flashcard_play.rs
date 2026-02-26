//! Flashcard play page

use crate::api;
use crate::components::intello::flashcard::flashcard_player::FlashcardPlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Flashcard play page — receives id from route
pub fn FlashcardPlayPage(id: String) -> Element {
    let fetch: FetchSetById<_> =
        FetchSetById(Arc::new(|id: String| {
            Box::pin(async move {
                api::flashcards::get_flashcard_sets()
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
                FlashcardPlayer {
                    set: s,
                    on_back: on_back,
                }
            }
        }));

    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/intello/flashcards",
            fetch_set: fetch,
            render_player: render,
        }
    }
}
