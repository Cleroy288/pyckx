//! Fill-blank list page — thin wrapper over `GameListPage`.

use crate::fill_blanks::api;
use crate::game_engine::{FetchSets, GameListPage};
use dioxus::prelude::*;
use std::sync::Arc;

/// Page title shown in the hero banner.
const TITLE: &str = "Fill Blank Sets";
/// Empty-state label.
const LABEL: &str = "fill_blanks";
/// Path to the generation page.
const GENERATE_PATH: &str = "/fill-blanks/generate";
/// Prefix for the per-set play page.
const PLAY_PREFIX: &str = "/fill-blanks/play";

/// List every fill-blank set the user owns.
pub fn FillBlankListPage() -> Element {
    let fetch = build_fetch();
    rsx! {
        GameListPage {
            title: TITLE,
            game_label: LABEL,
            generate_path: GENERATE_PATH,
            play_path_prefix: PLAY_PREFIX,
            fetch_sets: fetch,
        }
    }
}

/// Wrap `api::get_sets` into the `FetchSets` callback.
fn build_fetch() -> FetchSets<crate::fill_blanks::types::FillBlankSet>
{
    FetchSets(Arc::new(|| Box::pin(api::get_sets())))
}
