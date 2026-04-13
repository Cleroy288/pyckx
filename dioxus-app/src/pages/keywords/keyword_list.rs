//! Keywords list page

use crate::api;
use crate::components::game_shared::game_list_page::{
    FetchSets, GameListPage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Keywords list page
pub fn KeywordsListPage() -> Element {
    let fetch: FetchSets<_> =
        FetchSets(Arc::new(|| {
            Box::pin(
                api::keywords::get_keyword_sets(),
            )
        }));

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
