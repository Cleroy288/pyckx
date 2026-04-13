//! Demo C — Hybrid (brutalist accents, soft cards).

use super::shared::{demo_shell, HeroButtons, Variant};
use dioxus::prelude::*;

/// Demo C page — hybrid style.
pub fn DemoCPage() -> Element {
    let variant = Variant {
        prefix: "dc",
        hero_primary_class: "dc-btn-accent",
        hero_secondary_class: "dc-btn-outline",
        label: "Style C — Hybrid",
    };
    let buttons = HeroButtons {
        primary: "Generate",
        secondary: "Create",
    };
    demo_shell(variant, buttons)
}
