//! True/False generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// True/False generation page
pub fn TrueFalseGeneratePage() -> Element {
    let api_call: ApiCall =
        ApiCall(Arc::new(|data| {
            Box::pin(async move {
                let form = build_form_data(&data);
                api::true_false::create_true_false(
                    &form,
                )
                .await
                .map(|_| ())
            })
        }));

    rsx! {
        GameGeneratePage {
            title: "Generate TrueFalse",
            success_msg: "Generated!",
            redirect: "/intello/true-false",
            api_call: api_call,
        }
    }
}
