//! NavCard — Clickable navigation tile with icon

use crate::components::ui::icon::Icon;
use dioxus::prelude::*;

/// Navigation card classes
const CARD_CLS: &str = "\
    border-spin-card \
    flex flex-col items-center gap-3 \
    py-8 px-6 w-full \
    bg-[var(--glass-bg)] \
    backdrop-blur-[20px] \
    border border-[var(--color-border)] \
    text-[var(--color-text-primary)] \
    cursor-pointer font-[inherit]";

/// Icon container classes
const ICON_CLS: &str = "\
    w-14 h-14 flex items-center justify-center \
    bg-[color-mix(in_srgb,var(--primary)_10%,transparent)] \
    border border-[color-mix(in_srgb,var(--primary)_15%,transparent)] \
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
