// ** qcm_generate.rs **
// ==> QCM generation page (thin wrapper)

use crate::api;
use crate::components::intello::shared::game_generate_page::{
    ApiCall, GameGeneratePage,
};
use crate::domain::qcm_types::CreateQcmSetRequest;
use leptos::prelude::*;
use std::sync::Arc;

/// QCM generation page
#[component]
pub fn QcmGeneratePage() -> impl IntoView {
    let api_call: ApiCall = Arc::new(|data| {
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
    });

    view! {
        <GameGeneratePage
            title="Generate Qcm"
            success_msg="QCM generated"
            redirect="/intello/qcm"
            api_call=api_call
        />
    }
}
