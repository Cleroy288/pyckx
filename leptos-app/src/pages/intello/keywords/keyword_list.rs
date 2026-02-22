use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Keywords list page
#[component]
pub fn KeywordsListPage() -> impl IntoView {
    let fetch: FetchSets<_> = Arc::new(|| {
        Box::pin(
            api::keywords::get_keyword_sets(),
        )
    });

    view! {
        <GameListPage
            title="Keywords Sets"
            game_label="keywords"
            generate_path="/intello/keywords/generate"
            play_path_prefix="/intello/keywords/play"
            fetch_sets=fetch
        />
    }
}
