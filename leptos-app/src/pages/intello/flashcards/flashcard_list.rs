use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Flashcard list page
#[component]
pub fn FlashcardListPage() -> impl IntoView {
    let fetch: FetchSets<_> = Arc::new(|| {
        Box::pin(
            api::flashcards::get_flashcard_sets(),
        )
    });

    view! {
        <GameListPage
            title="Flashcard Sets"
            game_label="flashcards"
            generate_path="/intello/flashcards/generate"
            play_path_prefix="/intello/flashcards/play"
            fetch_sets=fetch
        />
    }
}
