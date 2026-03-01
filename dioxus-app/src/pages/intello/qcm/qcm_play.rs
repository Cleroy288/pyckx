//! QCM play page — thin wrapper over GamePlayPage

use crate::api;
use crate::components::intello::qcm::qcm_player::QcmPlayer;
use crate::components::intello::shared::game_play_page::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// QCM play page — receives id from route
pub fn QcmPlayPage(id: String) -> Element {
    let fetch: FetchSetById<_> =
        FetchSetById(Arc::new(|id: String| {
            Box::pin(async move {
                api::intello::get_qcm_set(&id)
                    .await
                    .ok()
                    .flatten()
            })
        }));

    let render: PlayerRenderer<_> =
        PlayerRenderer(Arc::new(|s, on_back| {
            rsx! {
                QcmPlayer {
                    set: s,
                    on_back: on_back,
                }
            }
        }));

    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/intello/qcm",
            fetch_set: fetch,
            render_player: render,
        }
    }
}
