//! AppGrid — grid container for app cards

use dioxus::prelude::*;

/// Grid section with title for app cards
#[component]
pub fn AppGrid(
    /// Section heading (defaults to "Your Apps")
    #[props(default = "Your Apps".to_string())]
    title: String,
    /// Grid content (app cards)
    children: Element,
) -> Element {
    rsx! {
        section {
            class: "max-w-[1200px] mx-auto",
            h2 {
                class: "text-2xl font-semibold \
                    text-[var(--color-text-primary)] \
                    mb-6 pl-2 \
                    border-l-4 \
                    border-[var(--color-primary)]",
                "{title}"
            }
            div {
                class: "grid \
                    grid-cols-[repeat(auto-fit,minmax(min(320px,100%),1fr))] \
                    gap-8 \
                    max-md:gap-6",
                {children}
            }
        }
    }
}
