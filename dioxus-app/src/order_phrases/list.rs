//! Order Phrases — list page (route `/order-phrases`).

use std::sync::Arc;

use dioxus::prelude::*;

use crate::game_engine::{FetchSets, GameListPage};

use super::api;
use super::types::OrderPhraseSet;

/// Page title shown above the set grid.
const TITLE: &str = "Order Phrase Sets";

/// Game label used for analytics + i18n.
const GAME_LABEL: &str = "order_phrases";

/// Path to the AI generation page.
const GENERATE_PATH: &str = "/order-phrases/generate";

/// Prefix for play routes (`{prefix}/{set_id}`).
const PLAY_PATH_PREFIX: &str = "/order-phrases/play";

/// List every order phrase set the user owns.
pub fn OrderPhraseListPage() -> Element {
    let fetch_sets = build_fetch_sets();

    rsx! {
        GameListPage {
            title: TITLE,
            game_label: GAME_LABEL,
            generate_path: GENERATE_PATH,
            play_path_prefix: PLAY_PATH_PREFIX,
            fetch_sets: fetch_sets,
        }
    }
}

/// Wrap the API call in the shared `FetchSets` adapter.
fn build_fetch_sets() -> FetchSets<OrderPhraseSet> {
    FetchSets(Arc::new(|| Box::pin(api::fetch_sets())))
}
