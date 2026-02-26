//! SectionDivider — Centered label with lines

use dioxus::prelude::*;

/// Horizontal divider with a centered text label
/// (line -- LABEL -- line)
#[component]
pub fn SectionDivider(label: String) -> Element {
    rsx! {
        div {
            class: "flex items-center gap-3 \
                text-[0.8rem] font-semibold \
                text-[var(--color-primary)] \
                uppercase tracking-wider \
                whitespace-nowrap",
            span {
                class: "flex-1 h-px \
                    bg-[color-mix(in_srgb,var(--primary)_25%,transparent)]",
            }
            "{label}"
            span {
                class: "flex-1 h-px \
                    bg-[color-mix(in_srgb,var(--primary)_25%,transparent)]",
            }
        }
    }
}
