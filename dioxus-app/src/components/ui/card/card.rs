//! Card — Glass/Solid/Popular card wrapper

use dioxus::prelude::*;

/// Card visual variant
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CardVariant {
    #[default]
    Glass,
    Solid,
    Popular,
}

/// Base card classes
const CARD_BASE: &str = "\
    relative p-10 px-8 overflow-hidden \
    transition-all duration-500 ease-out \
    will-change-transform \
    translate-x-0 translate-y-0 \
    hover:-translate-y-1";

/// Tailwind classes per variant
fn variant_class(v: CardVariant) -> &'static str {
    match v {
        CardVariant::Glass => "\
            bg-[var(--glass-bg)] \
            backdrop-blur-3xl \
            border border-[color-mix(in_srgb,var(--color-primary)_20%,transparent)] \
            shadow-[0_8px_32px_color-mix(in_srgb,var(--color-primary)_10%,transparent)] \
            hover:shadow-[0_12px_45px_color-mix(in_srgb,var(--color-primary)_15%,transparent)] \
            hover:border-[color-mix(in_srgb,var(--color-primary)_40%,transparent)] \
            hover:bg-[color-mix(in_srgb,var(--glass-bg)_90%,transparent)]",
        CardVariant::Solid => "\
            bg-[var(--color-background)] \
            border border-[color-mix(in_srgb,var(--color-primary)_20%,transparent)] \
            shadow-[0_4px_24px_color-mix(in_srgb,var(--color-primary)_10%,transparent)] \
            hover:shadow-xl",
        CardVariant::Popular => "\
            bg-[color-mix(in_srgb,var(--color-background)_60%,transparent)] \
            backdrop-blur-3xl \
            border-spin-always \
            shadow-xl \
            hover:-translate-y-2 \
            hover:shadow-[0_20px_60px_color-mix(in_srgb,var(--color-primary)_35%,transparent)]",
    }
}

/// Reusable card wrapper component
#[component]
pub fn Card(
    /// Card body content
    children: Element,
    /// Visual variant
    #[props(default)]
    variant: CardVariant,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let cls = format!(
        "{} {} {}",
        CARD_BASE,
        variant_class(variant),
        class,
    );

    rsx! {
        div { class: "{cls}",
            {children}
        }
    }
}
