//! PricingSection — single pricing box

use crate::components::vitrine::{CtaButton, CtaVariant};
use dioxus::prelude::*;

use super::data::PRICE_FEATURES;

/// Pricing section with single plan box
#[component]
pub fn PricingSection() -> Element {
    rsx! {
        section {
            class: "pricing-wrap",
            id: "pricing",
            h2 {
                "100% gratuit." br {} "Tout inclus."
            }
            div { class: "pricing-box",
                div { class: "top-tag",
                    "\u{2726} ACCES GRATUIT"
                }
                PriceDisplay {}
                p {
                    class: "text-[var(--muted)] \
                        text-[15px]",
                    "Toutes les apps. \
                        Toutes les features. \
                        Pour toujours."
                }
                FeaturesList {}
                CtaButton {
                    label: "Commencer ",
                    highlight: "gratuitement \u{2192}",
                    to: "/login",
                    variant: CtaVariant::Full,
                }
                p {
                    class: "text-xs \
                        text-[var(--muted)] mt-4",
                    "Aucune carte requise."
                }
            }
        }
    }
}

/// Big price display (euro symbol + amount)
#[component]
fn PriceDisplay() -> Element {
    rsx! {
        div { class: "big-price",
            "Gratuit"
        }
    }
}

/// Grid of pricing features with checkmarks
#[component]
fn FeaturesList() -> Element {
    rsx! {
        ul { class: "price-feats",
            for (index, feat) in PRICE_FEATURES.iter().enumerate() {
                li { key: "{index}", "{feat}" }
            }
        }
    }
}
