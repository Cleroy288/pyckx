//! Skeleton — Pulsing placeholder for loading states

use dioxus::prelude::*;

/// Shimmer skeleton classes.
/// Uses animate-pulse as Tailwind built-in.
/// Custom shimmer gradient via arbitrary CSS.
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
    let cls = format!("{} {}", SKELETON_CLS, class);

    rsx! {
        div {
            class: "{cls}",
            style: "width:{width};height:{height}",
        }
    }
}

/// Card-shaped skeleton placeholder
#[component]
pub fn SkeletonCard(
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let cls = format!(
        "flex flex-col gap-3 p-5 \
         rounded-[var(--radius-md,8px)] \
         border border-[var(--color-border,#333)] \
         bg-[var(--color-surface,#1a1a2e)] {}",
        class,
    );

    rsx! {
        div { class: "{cls}",
            Skeleton {
                height: "1.25rem".to_string(),
                width: "60%".to_string(),
            }
            Skeleton {
                height: "0.875rem".to_string(),
            }
            Skeleton {
                height: "0.875rem".to_string(),
                width: "80%".to_string(),
            }
        }
    }
}
