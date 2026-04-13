//! Keywords play page — loads a set then runs the player.

use crate::game_engine::{
    FetchSetById, GamePlayPage, PlayerRenderer,
};
use crate::keywords::api;
use crate::keywords::player::KeywordsPlayer;
use crate::keywords::types::KeywordSet;
use dioxus::prelude::*;
use std::sync::Arc;

/// Play page for a single keyword set, identified by `id`.
pub fn KeywordsPlayPage(id: String) -> Element {
    let fetch = build_fetch();
    let render = build_renderer();
    rsx! {
        GamePlayPage {
            id: id,
            back_path: "/keywords",
            fetch_set: fetch,
            render_player: render,
        }
    }
}

/// Fetch a single set by ID from the list endpoint.
fn build_fetch() -> FetchSetById<KeywordSet> {
    FetchSetById(Arc::new(|id: String| {
        Box::pin(async move {
            api::get_keyword_sets()
                .await
                .ok()
                .and_then(|sets| find_by_id(sets, &id))
        })
    }))
}

/// Build the player renderer used after fetch completes.
fn build_renderer() -> PlayerRenderer<KeywordSet> {
    PlayerRenderer(Arc::new(|set, on_back| {
        rsx! {
            KeywordsPlayer { set: set, on_back: on_back }
        }
    }))
}

/// Find a set by ID inside an in-memory list.
fn find_by_id(
    sets: Vec<KeywordSet>,
    id: &str,
) -> Option<KeywordSet> {
    sets.into_iter().find(|s| s.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keywords::types::{
        Keyword, KeywordQuestion,
    };

    fn make_set(id: &str) -> KeywordSet {
        KeywordSet {
            id: id.into(),
            name: "S".into(),
            description: "".into(),
            level: "easy".into(),
            language: "en".into(),
            subjects: vec![],
            questions: vec![KeywordQuestion {
                id: "q".into(),
                statement: "s".into(),
                explanation: "e".into(),
                keywords: vec![Keyword {
                    id: "k".into(),
                    word: "w".into(),
                    is_correct: true,
                }],
            }],
        }
    }

    #[test]
    fn test_find_by_id_returns_match() {
        let sets =
            vec![make_set("a"), make_set("b")];
        let found = find_by_id(sets, "b");
        assert_eq!(found.unwrap().id, "b");
    }

    #[test]
    fn test_find_by_id_returns_none_when_missing() {
        let sets = vec![make_set("a")];
        let found = find_by_id(sets, "zz");
        assert!(found.is_none());
    }
}
