use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Fill blanks list page
#[component]
pub fn FillBlankListPage() -> impl IntoView {
    let fetch: FetchSets<_> = Arc::new(|| {
        Box::pin(
            api::fill_blanks::get_fill_blank_sets(),
        )
    });

    view! {
        <GameListPage
            title="Fill Blank Sets"
            game_label="fill_blanks"
            generate_path="/intello/fill-blanks/generate"
            play_path_prefix="/intello/fill-blanks/play"
            fetch_sets=fetch
        />
    }
}
