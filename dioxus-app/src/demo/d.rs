//! Demo D — Clean + Primary border accent.

use super::shared::{demo_shell, HeroButtons, Variant};
use dioxus::prelude::*;

/// Demo D page — clean with primary border.
pub fn DemoDPage() -> Element {
    let variant = Variant {
        prefix: "dd",
        hero_primary_class: "dd-btn",
        hero_secondary_class: "dd-btn-sec",
        label: "Style D — Clean + Primary Border",
    };
    let buttons = HeroButtons {
        primary: "Generate",
        secondary: "Create",
    };
    demo_shell(variant, buttons)
}
