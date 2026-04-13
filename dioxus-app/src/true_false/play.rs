//! True/False play page — thin wrapper over `GamePlayPage`.

use std::sync::Arc;

use dioxus::prelude::*;

use crate::game_engine::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};

use super::api;
use super::player::TrueFalsePlayer;
use super::types::TrueFalseSet;

const BACK_PATH: &str = "/true-false";

/// True/False play page — fetches a set by id and plays it.
pub fn TrueFalsePlayPage(id: String) -> Element {
    let fetch = make_fetch();
    let render = make_renderer();
    rsx! {
        GamePlayPage::<TrueFalseSet> {
            id: id,
            back_path: BACK_PATH,
            fetch_set: fetch,
            render_player: render,
        }
    }
}

/// Build the async fetch-by-id closure.
fn make_fetch() -> FetchSetById<TrueFalseSet> {
    FetchSetById(Arc::new(|id: String| {
        Box::pin(api::find_set_by_id(id))
    }))
}

/// Build the renderer that mounts the True/False player.
fn make_renderer() -> PlayerRenderer<TrueFalseSet> {
    PlayerRenderer(Arc::new(|set, on_back| {
        rsx! {
            TrueFalsePlayer { set: set, on_back: on_back }
        }
    }))
}
