//! AppCard — clickable card for dashboard apps

use crate::components::ui::card::{Card, CardVariant};
use crate::components::ui::icon::Icon;
use dioxus::prelude::*;

/// Clickable app card that navigates on click
#[component]
pub fn AppCard(
    /// Card title
    title: String,
    /// Short description
    description: String,
    /// Icon name (Icon component)
    icon: String,
    /// Navigation path
    href: String,
) -> Element {
    rsx! {
        Link {
            to: "{href}",
            class: "no-underline block",
            Card {
                variant: CardVariant::Glass,
                class: "px-8 py-10 text-center \
                    cursor-pointer \
                    transition-all duration-300 \
                    hover:shadow-xl".to_string(),
                div {
                    class: "w-20 h-20 mx-auto mb-6 \
                        bg-[color-mix(in_srgb,var(--primary)_8%,transparent)] \
                        flex items-center justify-center \
                        text-[var(--color-primary)] \
                        transition-all duration-300 \
                        [&_svg]:w-10 [&_svg]:h-10",
                    Icon { icon_name: icon }
                }
                h3 {
                    class: "text-2xl font-bold \
                        text-[var(--color-text-primary)] \
                        mb-3",
                    "{title}"
                }
                p {
                    class: "text-base \
                        text-[var(--color-text-secondary)] \
                        leading-relaxed m-0",
                    "{description}"
                }
            }
        }
    }
}
