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
    bg-[var(--card)] rounded-[12px] \
    border-[1.5px] border-[var(--border)] \
    transition-[transform,border-color] \
    duration-200 ease-out \
    hover:-translate-y-0.5";

/// Tailwind classes per variant
fn variant_class(v: CardVariant) -> &'static str {
    match v {
        CardVariant::Glass => "\
            hover:border-[var(--color-primary)]",
        CardVariant::Solid => "\
            hover:border-[var(--color-primary)]",
        CardVariant::Popular => "\
            border-[var(--color-primary)] \
            hover:-translate-y-1",
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
