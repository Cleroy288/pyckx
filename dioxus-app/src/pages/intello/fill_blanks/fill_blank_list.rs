//! Fill blanks list page

use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Fill blanks list page
pub fn FillBlankListPage() -> Element {
    let fetch: FetchSets<_> =
        FetchSets(Arc::new(|| {
            Box::pin(
                api::fill_blanks::get_fill_blank_sets(),
            )
        }));

    rsx! {
        GameListPage {
            title: "Fill Blank Sets",
            game_label: "fill_blanks",
            generate_path: "/intello/fill-blanks/generate",
            play_path_prefix: "/intello/fill-blanks/play",
            fetch_sets: fetch,
        }
    }
}
