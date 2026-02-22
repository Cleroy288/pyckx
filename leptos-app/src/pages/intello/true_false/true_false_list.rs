use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// True/False list page
#[component]
pub fn TrueFalseListPage() -> impl IntoView {
    let fetch: FetchSets<_> = Arc::new(|| {
        Box::pin(
            api::true_false::get_true_false_sets(),
        )
    });

    view! {
        <GameListPage
            title="TrueFalse Sets"
            game_label="True / False"
            generate_path="/intello/true-false/generate"
            play_path_prefix="/intello/true-false/play"
            fetch_sets=fetch
        />
    }
}
