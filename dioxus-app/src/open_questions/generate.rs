//! Open-question generate page — wraps GameGeneratePage.

use super::api;
use crate::game_engine::{
    build_form_data, ApiCall, GameGeneratePage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Page constants — single source of truth.
const TITLE: &str = "Generate OpenQuestion";
const SUCCESS_MSG: &str = "Generated!";
const REDIRECT: &str = "/open-questions";

/// Generate page: AI-creates a set from the upload form.
pub fn OpenQuestionGeneratePage() -> Element {
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

/// Build the async API call passed to the generic page.
fn build_api_call() -> ApiCall {
    ApiCall(Arc::new(|data| {
        Box::pin(async move {
            let form = build_form_data(&data);
            api::create_set(&form).await.map(|_| ())
        })
    }))
}
