//! Skeleton — Pulsing placeholder for loading states

use dioxus::prelude::*;

/// Shimmer skeleton classes.
/// Uses animate-pulse as Tailwind built-in.
const SKELETON_CLS: &str = "\
    rounded-[var(--radius-sm,6px)] \
    animate-pulse \
    bg-[var(--color-border,#333)]";

/// Generic pulsing skeleton placeholder
#[component]
pub fn Skeleton(
    /// CSS width (e.g., "100%", "60%")
    #[props(default = "100%".to_string())]
    width: String,
    /// CSS height (e.g., "1rem", "1.25rem")
    #[props(default = "1rem".to_string())]
    height: String,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let cls =
        format!("{} {}", SKELETON_CLS, class);

    rsx! {
        div {
            class: "{cls}",
            style: "width:{width};height:{height}",
        }
    }
}
