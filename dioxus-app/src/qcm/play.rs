//! QCM play page — thin wrapper over `GamePlayPage`.

use std::sync::Arc;

use dioxus::prelude::*;

use crate::game_engine::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use crate::qcm::api;
use crate::qcm::player::QcmPlayer;
use crate::qcm::types::QcmSet;

const BACK_PATH: &str = "/qcm";

/// QCM play page — fetches a set by id and plays it.
pub fn QcmPlayPage(id: String) -> Element {
    let fetch = make_fetch();
    let render = make_renderer();
    rsx! {
        GamePlayPage::<QcmSet> {
            id: id,
            back_path: BACK_PATH,
            fetch_set: fetch,
            render_player: render,
        }
    }
}

/// Build the async fetch-by-id closure.
fn make_fetch() -> FetchSetById<QcmSet> {
    FetchSetById(Arc::new(|id: String| {
        Box::pin(async move {
            api::get_qcm_set(&id).await.ok().flatten()
        })
    }))
}

/// Build the renderer that mounts the QCM player.
fn make_renderer() -> PlayerRenderer<QcmSet> {
    PlayerRenderer(Arc::new(|set, on_back| {
        rsx! {
            QcmPlayer { set: set, on_back: on_back }
        }
    }))
}
