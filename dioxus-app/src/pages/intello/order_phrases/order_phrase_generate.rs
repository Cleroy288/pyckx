//! Order Phrase generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Order Phrase generation page
pub fn OrderPhraseGeneratePage() -> Element {
    let api_call: ApiCall =
        ApiCall(Arc::new(|data| {
            Box::pin(async move {
                let form = build_form_data(&data);
                api::order_phrases::create_order_phrases(
                    &form,
                )
                .await
                .map(|_| ())
            })
        }));

    rsx! {
        GameGeneratePage {
            title: "Generate OrderPhrase",
            success_msg: "Generated!",
            redirect: "/intello/order-phrases",
            api_call: api_call,
        }
    }
}
