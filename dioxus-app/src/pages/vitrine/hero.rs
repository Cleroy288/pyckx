//! HeroSection — headline, CTA, stats row

use dioxus::prelude::*;

use super::data::STATS;

/// SVG diagonal line for split-text effect
const SPLIT_LINE: &str = r##"<svg class="split-line"
  viewBox="0 0 100 100"
  preserveAspectRatio="none">
  <line x1="0" y1="46" x2="100" y2="54"/>
</svg>"##;

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
                    "\u{2726} Une plateforme \u{00B7} \
                        Plusieurs outils"
                }
            div { class: "split-wrap",
                span { class: "split-top", "PYCKX" }
                span {
                    class: "split-bottom",
                    "PYCKX"
                }
                div {
                    dangerous_inner_html: SPLIT_LINE,
                }
            }
            h1 {
                "One sub. "
                span { class: "c", "All apps." }
                " "
                span { class: "o", "Always." }
            }
            p { class: "desc",
                "Pyckx regroupe vos outils du \
                    quotidien sous un seul abonnement. \
                    Aujourd\u{2019}hui, "
                strong { "Intello" }
                " \u{2014} votre assistant \
                    d\u{2019}apprentissage AI. \
                    Demain, bien plus encore."
            }
            div { class: "hero-actions",
                Link { to: "/login",
                    span { class: "btn-ink",
                        "Commencer "
                        span { class: "cta-hi",
                            "gratuitement"
                        }
                    }
                }
                a { href: "#services",
                    class: "btn-ghost",
                    "Voir les apps \u{2192}"
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
            for stat in STATS.iter() {
                div { class: "stat-item",
                    h3 { "{stat.value}" }
                    p { "{stat.label}" }
                }
            }
        }
    }
}
