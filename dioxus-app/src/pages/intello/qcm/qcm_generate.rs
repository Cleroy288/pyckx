//! QCM generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    ApiCall, GameGeneratePage,
};
use crate::domain::qcm_types::CreateQcmSetRequest;
use dioxus::prelude::*;
use std::sync::Arc;

/// QCM generation page
pub fn QcmGeneratePage() -> Element {
    let api_call: ApiCall =
        ApiCall(Arc::new(|data| {
            Box::pin(async move {
                let req = CreateQcmSetRequest {
                    name: data.name,
                    description: data.description,
                    level: data.level,
                    language: data.language,
                    subjects: data.subjects,
                    questions: Vec::new(),
                };
                api::intello::create_qcm_set(req)
                    .await
                    .map(|_| ())
            })
        }));

    rsx! {
        GameGeneratePage {
            title: "Generate Qcm",
            success_msg: "QCM generated",
            redirect: "/intello/qcm",
            api_call: api_call,
        }
    }
}
