//! ItemCard — brutalist card for list items

use dioxus::prelude::*;

/// Card shell — neo-brutalist with offset shadow
const CARD_CLS: &str = "\
    item-card \
    flex flex-col gap-4 \
    p-6 h-full \
    bg-[var(--card)] \
    border-2 border-[var(--foreground)] \
    rounded-xl overflow-hidden \
    relative cursor-pointer \
    shadow-[4px_4px_0_var(--foreground)] \
    transition-all duration-200 \
    hover:-translate-y-1 \
    hover:shadow-[6px_6px_0_var(--foreground)]";

/// Actions row — inline buttons
const ACTIONS_CLS: &str = "\
    flex gap-3 mt-auto pt-2";

/// Brutalist card for list items
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
                    class: "item-card-title \
                        m-0 text-[1.125rem] \
                        text-[var(--foreground)] \
                        overflow-hidden text-ellipsis \
                        whitespace-nowrap",
                    "{name}"
                }
                {header_end}
            }
            div {
                class: "flex flex-col gap-2 flex-1",
                {children}
            }
            div { class: "{ACTIONS_CLS}",
                {actions}
            }
        }
    }
}
