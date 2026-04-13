//! Demo B — Clean / Soft (Quizlet-inspired).

use super::shared::{demo_shell, HeroButtons, Variant};
use dioxus::prelude::*;

/// Demo B page — clean Quizlet style.
pub fn DemoBPage() -> Element {
    let variant = Variant {
        prefix: "db",
        hero_primary_class: "db-btn",
        hero_secondary_class: "db-btn-sec",
        label: "Style B — Clean / Soft (Quizlet)",
    };
    let buttons = HeroButtons {
        primary: "Generate",
        secondary: "Create",
    };
    demo_shell(variant, buttons)
}
