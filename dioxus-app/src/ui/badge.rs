//! Badge — inline pill with a color variant.

use dioxus::prelude::*;

/// Color variant of a Badge.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Info,
}

/// Inline pill with a short label and a color variant.
#[component]
pub fn Badge(
    text: String,
    #[props(default)] variant: BadgeVariant,
    #[props(default)] class: String,
) -> Element {
    let full_class = build_class(variant, &class);
    rsx! { span { class: "{full_class}", "{text}" } }
}

/// Build the full CSS class string for a Badge.
fn build_class(variant: BadgeVariant, extra: &str) -> String {
    format!("{BASE} {} {}", variant_class(variant), extra)
}

/// Shared shell classes (shape, padding, font).
const BASE: &str = "\
    inline-flex items-center \
    py-1 px-2.5 text-xs font-semibold \
    leading-none whitespace-nowrap \
    rounded-[12px] bg-transparent";

/// Tailwind classes for a single variant.
fn variant_class(variant: BadgeVariant) -> &'static str {
    match variant {
        BadgeVariant::Default => "\
            text-[var(--color-text-secondary)] \
            border-[1.5px] border-[var(--color-border)]",
        BadgeVariant::Success => "\
            text-[var(--color-success-text)] \
            border-[1.5px] border-[var(--color-success-border)]",
        BadgeVariant::Warning => "\
            text-[var(--color-warning-text)] \
            border-[1.5px] border-[var(--color-warning-border)]",
        BadgeVariant::Error => "\
            text-[var(--color-error-text)] \
            border-[1.5px] border-[var(--color-error-border)]",
        BadgeVariant::Info => "\
            text-[var(--color-info-text)] \
            border-[1.5px] border-[var(--color-info-border)]",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_class_contains_base() {
        let cls = build_class(BadgeVariant::Default, "");
        assert!(cls.contains("inline-flex"));
    }

    #[test]
    fn test_build_class_appends_extra() {
        let cls = build_class(BadgeVariant::Info, "extra-x");
        assert!(cls.ends_with("extra-x"));
    }

    #[test]
    fn test_variant_class_success_has_success_token() {
        let cls = variant_class(BadgeVariant::Success);
        assert!(cls.contains("success-text"));
    }
}
