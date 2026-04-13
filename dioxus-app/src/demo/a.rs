//! Demo A — Neo-Brutalist variant.

use super::shared::{demo_shell, HeroButtons, Variant};
use dioxus::prelude::*;

/// Demo A page — neo-brutalist.
pub fn DemoAPage() -> Element {
    let variant = Variant {
        prefix: "da",
        hero_primary_class: "da-btn",
        hero_secondary_class: "da-btn-outline",
        label: "Style A — Neo-Brutalist",
    };
    let buttons = HeroButtons {
        primary: "Generate",
        secondary: "Create",
    };
    demo_shell(variant, buttons)
}
