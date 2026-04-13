//! Keywords sets list page.

use crate::game_engine::{FetchSets, GameListPage};
use crate::keywords::api;
use dioxus::prelude::*;
use std::sync::Arc;

/// List page for all keyword sets.
pub fn KeywordsListPage() -> Element {
    let fetch = build_fetch();
    rsx! {
        GameListPage {
            title: "Keywords Sets",
            game_label: "keywords",
            generate_path: "/keywords/generate",
            play_path_prefix: "/keywords/play",
            fetch_sets: fetch,
        }
    }
}

/// Build the async fetch callback used by the list page.
fn build_fetch() -> FetchSets<crate::keywords::types::KeywordSet>
{
    FetchSets(Arc::new(|| {
        Box::pin(api::get_keyword_sets())
    }))
}
