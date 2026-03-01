//! Flashcard list page

use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Flashcard list page
pub fn FlashcardListPage() -> Element {
    let fetch: FetchSets<_> =
        FetchSets(Arc::new(|| {
            Box::pin(
                api::flashcards::get_flashcard_sets(),
            )
        }));

    rsx! {
        GameListPage {
            title: "Flashcard Sets",
            game_label: "flashcards",
            generate_path: "/intello/flashcards/generate",
            play_path_prefix: "/intello/flashcards/play",
            fetch_sets: fetch,
        }
    }
}
