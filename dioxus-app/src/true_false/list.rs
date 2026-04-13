//! True/False list page — thin wrapper over `GameListPage`.

use std::sync::Arc;

use dioxus::prelude::*;

use crate::game_engine::{FetchSets, GameListPage};

use super::api;
use super::types::TrueFalseSet;

const TITLE: &str = "True / False Sets";
const LABEL: &str = "True / False";
const PATH_PLAY: &str = "/true-false/play";
const PATH_GENERATE: &str = "/true-false/generate";

/// True/False list page — shows every saved set.
pub fn TrueFalseListPage() -> Element {
    let fetch = make_fetch();
    rsx! {
        GameListPage::<TrueFalseSet> {
            title: TITLE,
            game_label: LABEL,
            generate_path: PATH_GENERATE,
            play_path_prefix: PATH_PLAY,
            fetch_sets: fetch,
        }
    }
}

/// Build the async fetch closure for the list page.
fn make_fetch() -> FetchSets<TrueFalseSet> {
    FetchSets(Arc::new(|| Box::pin(api::fetch_sets())))
}
