// ** badge.rs **
// ==> Inline pill with variant colors

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/badge/badge.module.css"
);

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

/// Inline pill badge
#[component]
pub fn Badge(
    #[prop(into)] text: String,
    #[prop(optional, default = BadgeVariant::Default)]
    variant: BadgeVariant,
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let variant_class = match variant {
        BadgeVariant::Default => style::default,
        BadgeVariant::Success => style::success,
        BadgeVariant::Warning => style::warning,
        BadgeVariant::Error => style::error,
        BadgeVariant::Info => style::info,
    };

    view! {
        <span class=format!(
            "{} {} {}",
            style::badge,
            variant_class,
            class.unwrap_or_default()
        )>
            {text}
        </span>
    }
}
