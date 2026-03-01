// ** keyword_generate.rs **
// ==> Keywords generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Keywords generation page
#[component]
pub fn KeywordsGeneratePage() -> impl IntoView {
    let api_call: ApiCall = Arc::new(|data| {
        Box::pin(async move {
            let form = build_form_data(&data);
            api::keywords::create_keywords(&form)
                .await
                .map(|_| ())
        })
    });

    view! {
        <GameGeneratePage
            title="Generate Keywords"
            success_msg="Generated!"
            redirect="/intello/keywords"
            api_call=api_call
        />
    }
}
