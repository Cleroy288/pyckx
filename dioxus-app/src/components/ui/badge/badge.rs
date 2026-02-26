//! Badge — Inline pill with variant colors

use dioxus::prelude::*;

/// Badge color variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Info,
}

/// Base badge classes
const BADGE_BASE: &str = "\
    inline-flex items-center \
    py-1 px-2.5 text-xs font-semibold \
    leading-none whitespace-nowrap \
    bg-transparent";

/// Tailwind classes per badge variant
fn variant_class(v: BadgeVariant) -> &'static str {
    match v {
        BadgeVariant::Default => "\
            text-[var(--color-text-secondary)] \
            border border-[var(--color-border)]",
        BadgeVariant::Success => "\
            text-[var(--color-success-text)] \
            border border-[var(--color-success-border)]",
        BadgeVariant::Warning => "\
            text-[var(--color-warning-text)] \
            border border-[var(--color-warning-border)]",
        BadgeVariant::Error => "\
            text-[var(--color-error-text)] \
            border border-[var(--color-error-border)]",
        BadgeVariant::Info => "\
            text-[var(--color-info-text)] \
            border border-[var(--color-info-border)]",
    }
}

/// Inline pill badge component
#[component]
pub fn Badge(
    /// Badge label text
    text: String,
    /// Color variant
    #[props(default)]
    variant: BadgeVariant,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let cls = format!(
        "{} {} {}",
        BADGE_BASE,
        variant_class(variant),
        class,
    );

    rsx! {
        span { class: "{cls}", "{text}" }
    }
}
