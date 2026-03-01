//! AppsSection — platform apps grid

use dioxus::prelude::*;

use super::data::INTELLO_FEATURES;

/// Platform section with app cards grid
#[component]
pub fn AppsSection() -> Element {
    rsx! {
        section {
            class: "vt-section",
            id: "services",
            div { class: "section-hd",
                h2 {
                    "Les apps" br {} "Pyckx."
                }
                p {
                    "Un abonnement unique vous donne \
                        acces a toutes nos applications, \
                        maintenant et dans le futur."
                }
            }
            div { class: "apps-grid",
                FeaturedCard {}
                AppCard {}
                ComingSoon {}
            }
        }
    }
}

/// Intello featured card (accent background)
#[component]
fn FeaturedCard() -> Element {
    rsx! {
        Link {
            to: "/produit/intello",
            class: "acard feat",
            div { class: "acard-icon",
                "\u{1F9E0}"
            }
            h3 { "Intello" }
            ul { class: "feat-ul",
                for feat in INTELLO_FEATURES.iter() {
                    li { "{feat}" }
                }
            }
            span { class: "badge",
                "App principale"
            }
        }
    }
}

/// Collection app card (regular style)
#[component]
fn AppCard() -> Element {
    rsx! {
        div { class: "acard",
            div { class: "acard-icon",
                "\u{1F4E6}"
            }
            h3 { "Collection" }
            p {
                "Gerez et organisez votre catalogue \
                    DVD personnel facilement."
            }
            span { class: "badge", "Disponible" }
        }
    }
}

/// Coming soon placeholder card
#[component]
fn ComingSoon() -> Element {
    rsx! {
        div { class: "acard soon",
            div {
                style: "font-size:36px;\
                    margin-bottom:12px",
                "\u{2726}"
            }
            h3 { "Bientot" }
            p {
                style: "font-size:12px;\
                    margin-top:8px",
                "De nouveaux outils arrivent \
                    regulierement."
            }
        }
    }
}
