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
            backdrop-blur-[25px] \
            border-[var(--glass-border)] \
            shadow-[0_4px_30px_color-mix(in_srgb,var(--primary)_5%,transparent)] \
            hover:shadow-xl \
            hover:border-[var(--glass-border-hover)] \
            hover:bg-[var(--glass-bg-hover)]",
        CardVariant::Solid => "\
            bg-[var(--color-background)] \
            border border-[color-mix(in_srgb,var(--primary)_15%,transparent)] \
            shadow-[0_4px_24px_color-mix(in_srgb,var(--primary)_8%,transparent)] \
            hover:shadow-lg",
        CardVariant::Popular => "\
            bg-[var(--glass-bg)] \
            backdrop-blur-[25px] \
            border-[3px] border-[var(--color-primary)] \
            shadow-lg \
            hover:-translate-y-1.5 \
            hover:shadow-[0_16px_50px_color-mix(in_srgb,var(--primary)_25%,transparent)]",
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
