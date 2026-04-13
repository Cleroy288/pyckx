//! Flashcard list page — wires the generic `GameListPage`
//! to the flashcards API.

use crate::flashcards::api;
use crate::game_engine::{FetchSets, GameListPage};
use dioxus::prelude::*;
use std::sync::Arc;

/// Hero title shown at the top of the list page.
const TITLE: &str = "Flashcard Sets";
/// Label used in the empty state ("No <label> yet").
const LABEL: &str = "flashcards";
/// Path to the AI-generation page.
const GENERATE_PATH: &str = "/flashcards/generate";
/// URL prefix for play (set ID is appended).
const PLAY_PREFIX: &str = "/flashcards/play";

/// Flashcard list page.
pub fn FlashcardListPage() -> Element {
    let fetch = fetch_sets();
    rsx! {
        GameListPage {
            title: TITLE,
            game_label: LABEL,
            generate_path: GENERATE_PATH,
            play_path_prefix: PLAY_PREFIX,
            fetch_sets: fetch,
        }
    }
}

/// Build the boxed async fetch passed to `GameListPage`.
fn fetch_sets() -> FetchSets<crate::flashcards::types::FlashcardSet> {
    FetchSets(Arc::new(|| {
        Box::pin(api::get_flashcard_sets())
    }))
}
