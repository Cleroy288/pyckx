//! NavCard — Clickable navigation tile with icon

use crate::components::ui::icon::Icon;
use dioxus::prelude::*;

/// Navigation card classes
const CARD_CLS: &str = "\
    flex flex-col items-center gap-3 \
    py-8 px-6 w-full \
    bg-[var(--card)] \
    border-[1.5px] border-[var(--border)] \
    rounded-[12px] \
    text-[var(--color-text-primary)] \
    cursor-pointer font-[inherit] \
    transition-all duration-200 \
    hover:-translate-y-0.5 \
    hover:border-[var(--color-primary)]";

/// Icon container classes
const ICON_CLS: &str = "\
    w-14 h-14 flex items-center justify-center \
    rounded-[12px] \
    bg-[var(--secondary)] \
    text-[var(--color-primary)] \
    transition-[background] \
    duration-[var(--transition-fast)] \
    [&_svg]:w-7 [&_svg]:h-7";

/// Clickable navigation tile component
#[component]
pub fn NavCard(
    /// Card title
    title: String,
    /// Short description below the title
    description: String,
    /// Icon name (from Icon component)
    icon: String,
    /// Click handler for navigation
    on_click: EventHandler<()>,
) -> Element {
    rsx! {
        button {
            class: "{CARD_CLS}",
            onclick: move |_| on_click.call(()),
            div { class: "{ICON_CLS}",
                Icon { icon_name: icon.clone() }
            }
            span {
                class: "text-[1.1rem] font-semibold",
                "{title}"
            }
            span {
                class: "text-[0.85rem] \
                    text-[var(--color-text-secondary)] \
                    leading-relaxed",
                "{description}"
            }
        }
    }
}
