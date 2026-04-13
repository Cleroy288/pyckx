//! Fill-blank play page — load by ID, render the player.

use crate::fill_blanks::api;
use crate::fill_blanks::player::FillBlankPlayer;
use crate::fill_blanks::types::FillBlankSet;
use crate::game_engine::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Path used when the player taps "back".
const BACK_PATH: &str = "/fill-blanks";

/// Play page bound to a fill-blank set ID.
pub fn FillBlankPlayPage(id: String) -> Element {
    let fetch = build_fetch();
    let render = build_renderer();
    rsx! {
        GamePlayPage {
            id: id,
            back_path: BACK_PATH,
            fetch_set: fetch,
            render_player: render,
        }
    }
}

/// Look the set up by ID inside the full list response.
fn build_fetch() -> FetchSetById<FillBlankSet> {
    FetchSetById(Arc::new(|wanted: String| {
        Box::pin(async move { find_set(wanted).await })
    }))
}

/// Fetch every set, then keep only the one with `id`.
async fn find_set(id: String) -> Option<FillBlankSet> {
    let sets = api::get_sets().await.ok()?;
    sets.into_iter().find(|s| s.id == id)
}

/// Render the player for a loaded fill-blank set.
fn build_renderer() -> PlayerRenderer<FillBlankSet> {
    PlayerRenderer(Arc::new(|set, on_back| {
        rsx! {
            FillBlankPlayer { set: set, on_back: on_back }
        }
    }))
}
