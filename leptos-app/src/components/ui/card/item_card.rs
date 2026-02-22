// ** item_card.rs **
// ==> Reusable list-item card: glass bg, flat,
//     header with name+badge, body, bottom actions.

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/card/item_card.module.css"
);

/// Flat glass card for list items (DVDs, game sets).
/// Header = name + header_end slot.
/// Body = children (domain-specific content).
/// Actions = button row pinned to the bottom.
#[component]
pub fn ItemCard(
    /// Card title (displayed as h3)
    #[prop(into)]
    name: String,
    /// Header-right slot (badge, level, etc.)
    header_end: AnyView,
    /// Action buttons pinned at bottom
    actions: AnyView,
    /// Body content between header and actions
    children: Children,
) -> impl IntoView {
    view! {
        <div class=style::card>
            <div class=style::header>
                <h3 class=style::name>{name}</h3>
                {header_end}
            </div>
            <div class=style::body>
                {children()}
            </div>
            <div class=style::actions>
                {actions}
            </div>
        </div>
    }
}
