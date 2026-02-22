// ** fill_blank_generate.rs **
// ==> Fill Blank generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Fill Blank generation page
#[component]
pub fn FillBlankGeneratePage() -> impl IntoView {
    let api_call: ApiCall = Arc::new(|data| {
        Box::pin(async move {
            let form = build_form_data(&data);
            api::fill_blanks::create_fill_blanks(&form)
                .await
                .map(|_| ())
        })
    });

    view! {
        <GameGeneratePage
            title="Generate FillBlank"
            success_msg="Generated!"
            redirect="/intello/fill-blanks"
            api_call=api_call
        />
    }
}
