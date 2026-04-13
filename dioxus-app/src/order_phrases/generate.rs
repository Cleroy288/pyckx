//! Order Phrases — AI generation page
//! (route `/order-phrases/generate`).

use std::sync::Arc;

use dioxus::prelude::*;

use crate::api::form_data::build_generate_form;
use crate::components::game_shared::generate_form::{
    GenerateFormData as LegacyFormData,
};
use crate::game_engine::{
    ApiCall, GameGeneratePage, GenerateFormData,
};

use super::api;

/// Hero banner title for the generate page.
const TITLE: &str = "Generate OrderPhrase";

/// Toast message shown on successful generation.
const SUCCESS_MSG: &str = "Generated!";

/// Where to navigate after a successful generation.
const REDIRECT: &str = "/order-phrases";

/// Generate new order phrase sets via AI.
pub fn OrderPhraseGeneratePage() -> Element {
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

/// Wrap `api::create_set` in the shared `ApiCall` adapter.
fn build_api_call() -> ApiCall {
    ApiCall(Arc::new(|data| {
        Box::pin(async move {
            let legacy = to_legacy_form(data);
            let form = build_generate_form(&legacy);
            api::create_set(&form).await.map(|_| ())
        })
    }))
}

/// Convert the engine form data into the legacy struct
/// expected by `build_generate_form`. Both share the
/// same field shape — this is a 1:1 copy.
fn to_legacy_form(
    data: GenerateFormData,
) -> LegacyFormData {
    LegacyFormData {
        name: data.name,
        description: data.description,
        instructions: data.instructions,
        language: data.language,
        level: data.level,
        subjects: data.subjects,
        num_questions: data.num_questions,
        files: data.files,
    }
}
