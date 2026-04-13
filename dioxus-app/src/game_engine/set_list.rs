//! Grid wrapper around game set cards with loading/empty.

use crate::ui::{CardGrid, EmptyState, SkeletonCard};
use dioxus::prelude::*;

/// List wrapper with loading, empty, and filled states.
#[component]
pub fn SetList(
    /// Whether the list is still loading.
    loading: Signal<bool>,
    /// Whether any items are available.
    has_items: Memo<bool>,
    /// Label used in the empty state ("QCM", "flashcard").
    game_label: String,
    /// Rendered set cards.
    children: Element,
) -> Element {
    rsx! {
        if loading() {
            CardGrid {
                SkeletonCard {}
                SkeletonCard {}
                SkeletonCard {}
            }
        } else if !has_items() {
            EmptyState {
                icon: "Layers",
                message: format!("No {} sets yet", game_label),
                description: "Create or generate your first set.",
            }
        } else {
            CardGrid { {children} }
        }
    }
}
