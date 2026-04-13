//! Pricing section for the landing page.

use dioxus::prelude::*;

/// Pricing section. Free tier for now.
#[component]
pub fn PricingSection() -> Element {
    rsx! {
        section { class: "vitrine-pricing",
            h2 { class: "vitrine-pricing-title", "Pricing" }
            p { class: "vitrine-pricing-desc",
                "Free while in beta."
            }
        }
    }
}
