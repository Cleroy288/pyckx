//! SkeletonCard — card-shaped loading placeholder

use super::Skeleton;
use dioxus::prelude::*;

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
