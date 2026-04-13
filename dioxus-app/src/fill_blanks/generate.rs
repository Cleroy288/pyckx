//! Fill-blank generation page — thin wrapper over
//! `GameGeneratePage`.

use crate::fill_blanks::api;
use crate::game_engine::{ApiCall, GameGeneratePage};
use dioxus::prelude::*;
use std::sync::Arc;

/// Page title shown in the hero banner.
const TITLE: &str = "Generate FillBlank";
/// Toast message after a successful generation.
const SUCCESS_MSG: &str = "Generated!";
/// Path to redirect to after success.
const REDIRECT: &str = "/fill-blanks";

/// Render the AI-generation page for fill-blank sets.
pub fn FillBlankGeneratePage() -> Element {
    let api_call = build_api_call();
    rsx! {
        GameGeneratePage {
            title: TITLE,
            success_msg: SUCCESS_MSG,
            redirect: REDIRECT,
            api_call: api_call,
        }
    }
}

/// Wrap `api::create` into the form's `ApiCall` callback.
fn build_api_call() -> ApiCall {
    ApiCall(Arc::new(|data| {
        Box::pin(async move {
            let form = api::build_form(&data);
            api::create(&form).await.map(|_| ())
        })
    }))
}
