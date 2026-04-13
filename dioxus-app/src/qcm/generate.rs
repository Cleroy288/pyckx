//! QCM generate page — thin wrapper over `GameGeneratePage`.

use std::sync::Arc;

use dioxus::prelude::*;

use crate::components::game_shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use crate::qcm::api;

const TITLE: &str = "Generate QCM";
const SUCCESS: &str = "QCM generated";
const REDIRECT: &str = "/qcm";

/// QCM generate page — AI generation from documents.
pub fn QcmGeneratePage() -> Element {
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
            api::generate_qcm(&form).await.map(|_| ())
        })
    }))
}
