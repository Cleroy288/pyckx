//! Open-question list page — thin wrapper over GameListPage.

use super::api;
use crate::game_engine::{FetchSets, GameListPage};
use dioxus::prelude::*;
use std::sync::Arc;

/// Path constants — single source of truth for routing.
const TITLE: &str = "Open Question Sets";
const LABEL: &str = "open_questions";
const GENERATE_PATH: &str = "/open-questions/generate";
const PLAY_PREFIX: &str = "/open-questions/play";

/// List page: shows every open-question set with play CTA.
pub fn OpenQuestionListPage() -> Element {
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

/// Build the async fetch passed to the generic list page.
fn build_fetch() -> FetchSets<super::types::OpenQuestionSet> {
    FetchSets(Arc::new(|| Box::pin(api::list_sets())))
}
