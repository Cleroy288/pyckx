//! Open Question generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Open Question generation page
pub fn OpenQuestionGeneratePage() -> Element {
    let api_call: ApiCall =
        ApiCall(Arc::new(|data| {
            Box::pin(async move {
                let form = build_form_data(&data);
                api::open_questions::create_open_questions(
                    &form,
                )
                .await
                .map(|_| ())
            })
        }));

    rsx! {
        GameGeneratePage {
            title: "Generate OpenQuestion",
            success_msg: "Generated!",
            redirect: "/intello/open-questions",
            api_call: api_call,
        }
    }
}
