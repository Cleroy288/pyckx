// ** card_grid.rs **
// ==> Responsive grid container for ItemCard lists

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/card/card_grid.module.css"
);

/// Responsive 3-column grid for card lists.
/// Cards stretch to equal height per row.
#[component]
pub fn CardGrid(children: Children) -> impl IntoView {
    view! {
        <div class=style::grid>
            {children()}
        </div>
    }
}
