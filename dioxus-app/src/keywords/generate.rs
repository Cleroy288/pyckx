//! Keywords AI generation page.

use crate::game_engine::{ApiCall, GameGeneratePage};
use crate::keywords::api;
use dioxus::prelude::*;
use std::sync::Arc;

/// Generation page for new keyword sets.
pub fn KeywordsGeneratePage() -> Element {
    let api_call = build_api_call();
    rsx! {
        GameGeneratePage {
            title: "Generate Keywords",
            success_msg: "Generated!",
            redirect: "/keywords",
            api_call: api_call,
        }
    }
}

/// Build the multipart-form API call used by the page.
fn build_api_call() -> ApiCall {
    ApiCall(Arc::new(|data| {
        Box::pin(async move {
            let form = api::build_form_data(&data);
            api::create_keywords(&form).await.map(|_| ())
        })
    }))
}
