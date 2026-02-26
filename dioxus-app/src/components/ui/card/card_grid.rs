//! CardGrid — Responsive grid container for cards

use dioxus::prelude::*;

/// Responsive 3-column grid for card lists.
/// Cards stretch to equal height per row.
#[component]
pub fn CardGrid(children: Element) -> Element {
    rsx! {
        div {
            class: "grid \
                grid-cols-[repeat(auto-fill,minmax(min(300px,100%),1fr))] \
                auto-rows-fr gap-5",
            {children}
        }
    }
}
