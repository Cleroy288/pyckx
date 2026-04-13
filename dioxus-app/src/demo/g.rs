//! Demo G — Brutalist Light (heavy accents, soft repeats).

use super::shared::{demo_shell, HeroButtons, Variant};
use dioxus::prelude::*;

/// Demo G page — brutalist light.
pub fn DemoGPage() -> Element {
    let variant = Variant {
        prefix: "dg",
        hero_primary_class: "dg-btn-primary",
        hero_secondary_class: "dg-btn-ghost",
        label: "Style G — Brutalist Light",
    };
    let buttons = HeroButtons {
        primary: "Generate",
        secondary: "Create",
    };
    demo_shell(variant, buttons)
}
