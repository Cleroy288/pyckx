// ** flashcard_generate.rs **
// ==> Flashcard generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::{
    game_generate_page::{
        build_form_data, ApiCall, GameGeneratePage,
    },
};
use leptos::prelude::*;
use std::sync::Arc;

/// Flashcard generation page
#[component]
pub fn FlashcardGeneratePage() -> impl IntoView {
    let api_call: ApiCall = Arc::new(|data| {
        Box::pin(async move {
            let form = build_form_data(&data);
            api::flashcards::create_flashcards(&form)
                .await
                .map(|_| ())
        })
    });

    view! {
        <GameGeneratePage
            title="Generate Flashcard"
            success_msg="Generated!"
            redirect="/intello/flashcards"
            api_call=api_call
        />
    }
}
