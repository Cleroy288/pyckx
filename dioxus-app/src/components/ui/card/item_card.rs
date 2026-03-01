//! ItemCard — Flat glass card for list items

use dioxus::prelude::*;

/// Card shell classes
const CARD_CLS: &str = "\
    border-spin-card \
    flex flex-col gap-2.5 \
    pt-6 px-6 pb-0 h-full \
    bg-[var(--glass-bg)] \
    backdrop-blur-[20px] \
    border border-[var(--color-border)]";

/// Actions row classes
const ACTIONS_CLS: &str = "\
    flex mt-auto -mx-6 \
    border-t border-[var(--color-border)] \
    [&_button]:flex-1 [&_button]:rounded-none \
    [&_button]:border-none [&_button]:shadow-none \
    [&_button+button]:border-l \
    [&_button+button]:border-[var(--color-border)]";

/// Flat glass card for list items
#[component]
pub fn ItemCard(
    /// Card title (displayed as h3)
    name: String,
    /// Header-right slot (badge, level, etc.)
    header_end: Element,
    /// Action buttons pinned at bottom
    actions: Element,
    /// Body content between header and actions
    children: Element,
) -> Element {
    rsx! {
        div { class: "{CARD_CLS}",
            div {
                class: "flex items-center \
                    justify-between gap-3",
                h3 {
                    class: "m-0 text-[1.1rem] \
                        font-semibold \
                        text-[var(--color-text-primary)] \
                        overflow-hidden text-ellipsis \
                        whitespace-nowrap",
                    "{name}"
                }
                {header_end}
            }
            div {
                class: "flex flex-col gap-1.5 flex-1",
                {children}
            }
            div { class: "{ACTIONS_CLS}",
                {actions}
            }
        }
    }
}
