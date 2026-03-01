//! PricingSection — single pricing box

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
                    style: "color:var(--muted);\
                        font-size:15px",
                    "Toutes les apps. \
                        Toutes les features. \
                        Pour toujours."
                }
                FeaturesList {}
                Link { to: "/login",
                    span { class: "btn-cta",
                        "Commencer "
                        span { class: "cta-hi",
                            "gratuitement \u{2192}"
                        }
                    }
                }
                p {
                    style: "font-size:12px;\
                        color:var(--muted);\
                        margin-top:16px",
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
            for feat in PRICE_FEATURES.iter() {
                li { "{feat}" }
            }
        }
    }
}
