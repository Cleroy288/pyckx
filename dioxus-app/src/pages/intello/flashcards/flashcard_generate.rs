//! Flashcard generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Flashcard generation page
pub fn FlashcardGeneratePage() -> Element {
    let api_call: ApiCall =
        ApiCall(Arc::new(|data| {
            Box::pin(async move {
                let form = build_form_data(&data);
                api::flashcards::create_flashcards(
                    &form,
                )
                .await
                .map(|_| ())
            })
        }));

    rsx! {
        GameGeneratePage {
            title: "Generate Flashcard",
            success_msg: "Generated!",
            redirect: "/intello/flashcards",
            api_call: api_call,
        }
    }
}
