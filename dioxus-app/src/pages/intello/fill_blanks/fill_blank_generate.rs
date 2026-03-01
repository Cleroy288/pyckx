//! Fill Blank generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Fill Blank generation page
pub fn FillBlankGeneratePage() -> Element {
    let api_call: ApiCall =
        ApiCall(Arc::new(|data| {
            Box::pin(async move {
                let form = build_form_data(&data);
                api::fill_blanks::create_fill_blanks(
                    &form,
                )
                .await
                .map(|_| ())
            })
        }));

    rsx! {
        GameGeneratePage {
            title: "Generate FillBlank",
            success_msg: "Generated!",
            redirect: "/intello/fill-blanks",
            api_call: api_call,
        }
    }
}
