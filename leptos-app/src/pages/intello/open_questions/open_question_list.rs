use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Open questions list page
#[component]
pub fn OpenQuestionListPage() -> impl IntoView {
    let fetch: FetchSets<_> = Arc::new(|| {
        Box::pin(
            api::open_questions::get_open_question_sets(),
        )
    });

    view! {
        <GameListPage
            title="Open Question Sets"
            game_label="open_questions"
            generate_path="/intello/open-questions/generate"
            play_path_prefix="/intello/open-questions/play"
            fetch_sets=fetch
        />
    }
}
