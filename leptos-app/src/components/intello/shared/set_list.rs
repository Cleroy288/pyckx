// ** set_list.rs **
// ==> Generic set list with loading/empty states

use crate::components::ui::card::CardGrid;
use crate::components::ui::empty_state::EmptyState;
use crate::components::ui::skeleton::SkeletonCard;
use leptos::prelude::*;

/// Generic list container for game sets
#[component]
pub fn SetList(
    /// Loading state
    #[prop(into)]
    loading: Signal<bool>,
    /// Whether there are items
    #[prop(into)]
    has_items: Signal<bool>,
    /// Game type label for empty state
    #[prop(into)]
    game_label: String,
    /// The rendered set cards
    children: Children,
) -> impl IntoView {
    let label = game_label.clone();

    view! {
        <div style:display=move || {
            if loading.get() { "block" }
            else { "none" }
        }>
            <CardGrid>
                <SkeletonCard />
                <SkeletonCard />
                <SkeletonCard />
            </CardGrid>
        </div>
        <div style:display=move || {
            if !loading.get() && !has_items.get() {
                "block"
            } else {
                "none"
            }
        }>
            <EmptyState
                icon="Layers".to_string()
                message=format!(
                    "No {} sets yet", label
                )
                description="Create or generate \
                    your first set."
                    .to_string()
            />
        </div>
        <div style:display=move || {
            if !loading.get() && has_items.get() {
                "block"
            } else {
                "none"
            }
        }>
            <CardGrid>
                {children()}
            </CardGrid>
        </div>
    }
}
