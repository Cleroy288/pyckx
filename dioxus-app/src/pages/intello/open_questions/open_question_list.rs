//! Open questions list page

use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use dioxus::prelude::*;
use std::sync::Arc;

/// Open questions list page
pub fn OpenQuestionListPage() -> Element {
    let fetch: FetchSets<_> =
        FetchSets(Arc::new(|| {
            Box::pin(
                api::open_questions::get_open_question_sets(),
            )
        }));

    rsx! {
        GameListPage {
            title: "Open Question Sets",
            game_label: "open_questions",
            generate_path: "/intello/open-questions/generate",
            play_path_prefix: "/intello/open-questions/play",
            fetch_sets: fetch,
        }
    }
}
