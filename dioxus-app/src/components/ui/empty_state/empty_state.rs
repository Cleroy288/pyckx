//! EmptyState — Centered icon + message + CTA

use crate::components::ui::icon::Icon;
use dioxus::prelude::*;

/// Empty state placeholder component
#[component]
pub fn EmptyState(
    /// Icon name to display
    icon: String,
    /// Main message text
    message: String,
    /// Optional description below message
    #[props(default)]
    description: Option<String>,
    /// Optional CTA slot
    #[props(default)]
    children: Element,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center \
                justify-center gap-3 \
                py-12 px-6 text-center {class}",
            div {
                class: "w-12 h-12 flex \
                    items-center justify-center \
                    text-[var(--color-text-secondary,#999)] \
                    opacity-50",
                Icon { icon_name: icon }
            }
            h3 {
                class: "m-0 text-base font-semibold \
                    text-[var(--color-text-primary)]",
                "{message}"
            }
            if let Some(desc) = &description {
                p {
                    class: "m-0 text-sm \
                        text-[var(--color-text-secondary,#999)] \
                        max-w-xs",
                    "{desc}"
                }
            }
            div { class: "mt-2",
                {children}
            }
        }
    }
}
