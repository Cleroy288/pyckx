use crate::api;
use crate::components::intello::qcm::qcm_player::QcmPlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use leptos::prelude::*;
use std::sync::Arc;

/// QCM play page — thin wrapper over GamePlayPage
#[component]
pub fn QcmPlayPage() -> impl IntoView {
    let fetch: FetchSetById<_> =
        Arc::new(|id: String| {
            Box::pin(async move {
                api::intello::get_qcm_set(&id)
                    .await
                    .ok()
                    .flatten()
            }) as _
        });

    let render: PlayerRenderer<_> =
        Arc::new(|s, on_back| {
            view! {
                <QcmPlayer set=s on_back=on_back />
            }
            .into_any()
        });

    view! {
        <GamePlayPage
            back_path="/intello/qcm"
            fetch_set=fetch
            render_player=render
        />
    }
}
