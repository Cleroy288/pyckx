//! Demo E — Clean + Top accent stripe.

use super::shared::{demo_shell, HeroButtons, Variant};
use dioxus::prelude::*;

/// Demo E page — clean with top accent stripe.
pub fn DemoEPage() -> Element {
    let variant = Variant {
        prefix: "de",
        hero_primary_class: "de-btn",
        hero_secondary_class: "de-btn-sec",
        label: "Style E — Clean + Top Accent Stripe",
    };
    let buttons = HeroButtons {
        primary: "Generate",
        secondary: "Create",
    };
    demo_shell(variant, buttons)
}
