//! HeroSection — headline, CTA, stats row

use crate::components::logo::SPLIT_LINE_SVG;
use crate::components::vitrine::{
    CtaButton, GhostButton, GhostTarget, HeroActions,
};
use dioxus::prelude::*;

use super::data::STATS;

/// Hero section: eyebrow, h1, desc, actions, stats
#[component]
pub fn HeroSection() -> Element {
    rsx! {
        header { class: "hero relative overflow-visible",
            // Ambient Orbs
            div { class: "ambient-orb ambient-orb-primary w-[500px] h-[500px] top-[-10%] left-[-15%]" }
            div { class: "ambient-orb ambient-orb-secondary w-[400px] h-[400px] bottom-[20%] right-[-10%]" }

            div { class: "relative z-10",
                div { class: "eyebrow bg-white/5 border border-white/10 rounded-full px-4 py-1.5 backdrop-blur-md",
                    "\u{2726} La plateforme \u{00B7} \
                        des etudiants"
                }
            div { class: "split-wrap mb-6",
                    span {
                        class: "split-top",
                        "PYCKX"
                    }
                    span {
                        class: "split-bottom",
                        "PYCKX"
                    }
                    div {
                        dangerous_inner_html:
                            SPLIT_LINE_SVG,
                    }
                }
            h1 {
                "Learn "
                span { class: "c", "smarter." }
                " "
                span { class: "o", "Play." }
            }
            p { class: "desc",
                "Transformez n\u{2019}importe quel \
                    document en "
                strong { "quiz interactifs" }
                " en quelques secondes. \
                    7 types de jeux, correction \
                    IA, progression suivie \u{2014} \
                    100% gratuit."
            }
            HeroActions {
                CtaButton {
                    label: "Commencer ",
                    highlight: "gratuitement",
                    to: "/login",
                }
                GhostButton {
                    label: "Voir les jeux \u{2192}",
                    target: GhostTarget::Anchor(
                        "#jeux"
                    ),
                }
            }
            StatsRow {}
            }
        }
    }
}

/// Row of key stats below hero CTA
#[component]
fn StatsRow() -> Element {
    rsx! {
        div { class: "stats-row",
            for (index, stat) in STATS.iter().enumerate() {
                div { key: "{index}", class: "stat-item",
                    h3 { "{stat.value}" }
                    p { "{stat.label}" }
                }
            }
        }
    }
}
