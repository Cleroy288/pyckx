// ** set_list.rs **
// ==> Generic set list with loading/empty states

use crate::components::ui::card::CardGrid;
use crate::components::ui::empty_state::EmptyState;
use crate::components::ui::skeleton::SkeletonCard;
use dioxus::prelude::*;

/// Generic list container for game sets
#[component]
pub fn SetList(
    /// Loading state
    loading: Signal<bool>,
    /// Whether there are items
    has_items: Memo<bool>,
    /// Game type label for empty state
    game_label: String,
    /// The rendered set cards
    children: Element,
) -> Element {
    rsx! {
        if (loading)() {
            CardGrid {
                SkeletonCard {}
                SkeletonCard {}
                SkeletonCard {}
            }
        } else if !(has_items)() {
            EmptyState {
                icon: "Layers",
                message: format!(
                    "No {} sets yet", game_label
                ),
                description: "Create or generate \
                    your first set.",
            }
        } else {
            CardGrid { {children} }
        }
    }
}
