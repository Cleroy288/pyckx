//! QCM generation page (thin wrapper)

use crate::api;
use crate::components::game_shared::game_generate_page::{
    build_form_data, ApiCall, GameGeneratePage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// QCM generation page
pub fn QcmGeneratePage() -> Element {
    let api_call: ApiCall =
        ApiCall(Arc::new(|data| {
            Box::pin(async move {
                let form = build_form_data(&data);
                api::study::generate_qcm(&form)
                    .await
                    .map(|_| ())
            })
        }));

    rsx! {
        GameGeneratePage {
            title: "Generate Qcm",
            success_msg: "QCM generated",
            redirect: "/qcm",
            api_call: api_call,
        }
    }
}
