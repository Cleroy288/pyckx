use crate::api;
use crate::components::intello::flashcard::flashcard_player::FlashcardPlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Flashcard play page
#[component]
pub fn FlashcardPlayPage() -> impl IntoView {
    let fetch: FetchSetById<_> =
        Arc::new(|id: String| {
            Box::pin(async move {
                api::flashcards::get_flashcard_sets()
                    .await
                    .ok()
                    .and_then(|v| {
                        v.into_iter()
                            .find(|s| s.id == id)
                    })
            }) as _
        });

    let render: PlayerRenderer<_> =
        Arc::new(|s, on_back| {
            view! {
                <FlashcardPlayer
                    set=s on_back=on_back
                />
            }
            .into_any()
        });

    view! {
        <GamePlayPage
            back_path="/intello/flashcards"
            fetch_set=fetch
            render_player=render
        />
    }
}
