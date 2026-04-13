//! Feedback primitives — Spinner, Skeleton, SkeletonCard,
//! EmptyState, LoadingBoundary. Toasts live in `toast.rs`.

pub use super::toast::{
    use_toast, ToastData, ToastProvider, ToastState, ToastVariant,
};

use super::icon::Icon;
use dioxus::prelude::*;

/// Size variant for the `Spinner`.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Rotating CSS spinner.
#[component]
pub fn Spinner(
    #[props(default)] size: SpinnerSize,
    #[props(default)] class: String,
) -> Element {
    let ring_cls = format!(
        "rounded-full animate-spin \
         border-[var(--color-border,#333)] \
         border-t-[var(--color-primary,#6366f1)] {}",
        spinner_size_class(size),
    );
    rsx! {
        div {
            class: "inline-flex items-center justify-center {class}",
            div { class: "{ring_cls}" }
        }
    }
}

/// Pulsing rectangle placeholder for loading states.
#[component]
pub fn Skeleton(
    #[props(default = "100%".to_string())] width: String,
    #[props(default = "1rem".to_string())] height: String,
    #[props(default)] class: String,
) -> Element {
    let cls = format!("{SKELETON_CLS} {}", class);
    rsx! {
        div {
            class: "{cls}",
            style: "width:{width};height:{height}",
        }
    }
}

/// Card-shaped skeleton (title + two lines).
#[component]
pub fn SkeletonCard(#[props(default)] class: String) -> Element {
    let cls = format!("{SKELETON_CARD_CLS} {}", class);
    rsx! {
        div { class: "{cls}",
            Skeleton { height: "1.25rem".to_string(), width: "60%".to_string() }
            Skeleton { height: "0.875rem".to_string() }
            Skeleton { height: "0.875rem".to_string(), width: "80%".to_string() }
        }
    }
}

/// Centered icon + message + optional CTA slot.
#[component]
pub fn EmptyState(
    icon: String,
    message: String,
    #[props(default)] description: Option<String>,
    #[props(default)] children: Element,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div { class: "{EMPTY_CLS} {class}",
            div { class: "{EMPTY_ICON_CLS}",
                Icon { icon_name: icon }
            }
            h3 { class: "m-0 text-base font-semibold \
                  text-[var(--color-text-primary)]",
                "{message}" }
            {render_description(description.as_deref())}
            div { class: "mt-2", {children} }
        }
    }
}

/// Shows a Spinner while `loading`, then renders children.
#[component]
pub fn LoadingBoundary(
    loading: Signal<bool>,
    children: Element,
) -> Element {
    let is_loading = *loading.read();
    rsx! {
        div {
            style: if is_loading { "display:block" } else { "display:none" },
            Spinner {}
        }
        div {
            style: if is_loading { "display:none" } else { "display:block" },
            {children}
        }
    }
}

/// Render a description paragraph when set.
fn render_description(desc: Option<&str>) -> Element {
    match desc {
        Some(text) => rsx! {
            p { class: "m-0 text-sm \
                 text-[var(--color-text-secondary,#999)] max-w-xs",
                "{text}" }
        },
        None => rsx! {},
    }
}

/// Tailwind classes for a single `SpinnerSize`.
fn spinner_size_class(size: SpinnerSize) -> &'static str {
    match size {
        SpinnerSize::Small => "w-4 h-4 border-2",
        SpinnerSize::Medium => "w-6 h-6 border-[3px]",
        SpinnerSize::Large => "w-10 h-10 border-4",
    }
}

const SKELETON_CLS: &str = "\
    rounded-[var(--radius-sm,6px)] \
    animate-pulse bg-[var(--color-border,#333)]";

const SKELETON_CARD_CLS: &str = "\
    flex flex-col gap-3 p-5 \
    rounded-[var(--radius-md,8px)] \
    border border-[var(--color-border,#333)] \
    bg-[var(--color-surface,#1a1a2e)]";

const EMPTY_CLS: &str = "\
    flex flex-col items-center justify-center \
    gap-3 py-12 px-6 text-center";

const EMPTY_ICON_CLS: &str = "\
    w-12 h-12 flex items-center justify-center \
    text-[var(--color-text-secondary,#999)] opacity-50";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_size_class_small_uses_w_4() {
        assert!(spinner_size_class(SpinnerSize::Small).contains("w-4"));
    }

    #[test]
    fn test_spinner_size_class_large_uses_w_10() {
        assert!(spinner_size_class(SpinnerSize::Large).contains("w-10"));
    }

    #[test]
    fn test_spinner_size_class_medium_uses_w_6() {
        assert!(spinner_size_class(SpinnerSize::Medium).contains("w-6"));
    }
}
