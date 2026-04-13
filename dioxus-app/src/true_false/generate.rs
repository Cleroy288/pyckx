//! True/False generate page — wrapper over `GameGeneratePage`.

use std::sync::Arc;

use dioxus::prelude::*;

use crate::game_engine::{
    build_form_data, ApiCall, GameGeneratePage,
};

use super::api;

const TITLE: &str = "Generate True / False";
const SUCCESS: &str = "True / False generated";
const REDIRECT: &str = "/true-false";

/// True/False generate page — AI generation from documents.
pub fn TrueFalseGeneratePage() -> Element {
    let api_call = make_api_call();
    rsx! {
        GameGeneratePage {
            title: TITLE,
            success_msg: SUCCESS,
            redirect: REDIRECT,
            api_call: api_call,
        }
    }
}

/// Build the submit closure consumed by `GameGeneratePage`.
fn make_api_call() -> ApiCall {
    ApiCall(Arc::new(|data| {
        Box::pin(async move {
            let form = build_form_data(&data);
            api::create_set(&form).await.map(|_| ())
        })
    }))
}
