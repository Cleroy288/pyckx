//! True/False list page

use crate::api;
use crate::components::game_shared::game_list_page::{
    FetchSets, GameListPage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// True/False list page
pub fn TrueFalseListPage() -> Element {
    let fetch: FetchSets<_> =
        FetchSets(Arc::new(|| {
            Box::pin(
                api::true_false::get_true_false_sets(),
            )
        }));

    rsx! {
        GameListPage {
            title: "TrueFalse Sets",
            game_label: "True / False",
            generate_path: "/true-false/generate",
            play_path_prefix: "/true-false/play",
            fetch_sets: fetch,
        }
    }
}
