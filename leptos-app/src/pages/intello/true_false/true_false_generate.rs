// ** true_false_generate.rs **
// ==> True/False generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// True/False generation page
#[component]
pub fn TrueFalseGeneratePage() -> impl IntoView {
    let api_call: ApiCall = Arc::new(|data| {
        Box::pin(async move {
            let form = build_form_data(&data);
            api::true_false::create_true_false(&form)
                .await
                .map(|_| ())
        })
    });

    view! {
        <GameGeneratePage
            title="Generate TrueFalse"
            success_msg="Generated!"
            redirect="/intello/true-false"
            api_call=api_call
        />
    }
}
