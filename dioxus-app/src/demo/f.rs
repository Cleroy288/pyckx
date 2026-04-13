//! Demo F — Clean + Paper cutout (strong elevation).

use super::shared::{demo_shell, HeroButtons, Variant};
use dioxus::prelude::*;

/// Demo F page — paper cutout elevation.
pub fn DemoFPage() -> Element {
    let variant = Variant {
        prefix: "df",
        hero_primary_class: "df-btn",
        hero_secondary_class: "df-btn-sec",
        label: "Style F — Paper Cutout (elevated)",
    };
    let buttons = HeroButtons {
        primary: "Generate",
        secondary: "Create",
    };
    demo_shell(variant, buttons)
}
