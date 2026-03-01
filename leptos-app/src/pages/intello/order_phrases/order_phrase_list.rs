use crate::api;
use crate::components::intello::shared::game_list_page::{
    FetchSets, GameListPage,
};
use leptos::prelude::*;
use std::sync::Arc;

/// Order phrases list page
#[component]
pub fn OrderPhraseListPage() -> impl IntoView {
    let fetch: FetchSets<_> = Arc::new(|| {
        Box::pin(
            api::order_phrases::get_order_phrase_sets(),
        )
    });

    view! {
        <GameListPage
            title="Order Phrase Sets"
            game_label="order_phrases"
            generate_path="/intello/order-phrases/generate"
            play_path_prefix="/intello/order-phrases/play"
            fetch_sets=fetch
        />
    }
}
