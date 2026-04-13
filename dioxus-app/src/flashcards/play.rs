//! Flashcard play page — wires the generic `GamePlayPage`
//! to the flashcards API and the `FlashcardPlayer`.

use crate::flashcards::api;
use crate::flashcards::player::FlashcardPlayer;
use crate::flashcards::types::FlashcardSet;
use crate::game_engine::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Path navigated to when the player asks to go back.
const BACK_PATH: &str = "/flashcards";

/// Flashcard play page — receives the set ID from route.
pub fn FlashcardPlayPage(id: String) -> Element {
    let fetch = fetch_set();
    let render = render_player();
    rsx! {
        GamePlayPage {
            id: id,
            back_path: BACK_PATH,
            fetch_set: fetch,
            render_player: render,
        }
    }
}

/// Build the boxed "fetch one set by id" call.
fn fetch_set() -> FetchSetById<FlashcardSet> {
    FetchSetById(Arc::new(|id: String| {
        Box::pin(async move { find_set_by_id(id).await })
    }))
}

/// Look up a single set by ID inside the full list.
/// Returns `None` if the request fails or the ID is absent.
async fn find_set_by_id(id: String) -> Option<FlashcardSet> {
    let sets = api::get_flashcard_sets().await.ok()?;
    sets.into_iter().find(|s| s.id == id)
}

/// Build the renderer that mounts `FlashcardPlayer`.
fn render_player() -> PlayerRenderer<FlashcardSet> {
    PlayerRenderer(Arc::new(|set, on_back| {
        rsx! {
            FlashcardPlayer { set: set, on_back: on_back }
        }
    }))
}
