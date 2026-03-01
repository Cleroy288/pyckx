//! Spinner — CSS-animated loading indicator

use dioxus::prelude::*;

/// Spinner size variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Ring dimension classes per size
fn size_class(s: SpinnerSize) -> &'static str {
    match s {
        SpinnerSize::Small => {
            "w-4 h-4 border-2"
        }
        SpinnerSize::Medium => {
            "w-6 h-6 border-[3px]"
        }
        SpinnerSize::Large => {
            "w-10 h-10 border-4"
        }
    }
}

/// Loading spinner component
#[component]
pub fn Spinner(
    /// Size variant
    #[props(default)]
    size: SpinnerSize,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let ring_cls = format!(
        "rounded-full animate-spin \
         border-[var(--color-border,#333)] \
         border-t-[var(--color-primary,#6366f1)] {}",
        size_class(size),
    );

    rsx! {
        div {
            class: "inline-flex items-center \
                justify-center {class}",
            div { class: "{ring_cls}" }
        }
    }
}
