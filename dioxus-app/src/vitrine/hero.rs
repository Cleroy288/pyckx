//! `HeroSection` — eyebrow, split title, headline, CTA, stats.
//!
//! One public component + four private visual blocks,
//! ordered top-down exactly as the reader sees the page.

use dioxus::prelude::*;

use super::buttons::{
    CtaButton, GhostButton, GhostTarget, HeroActions,
};
use super::data::STATS;
use crate::home::SPLIT_LINE_SVG;

/// Main hero block at the top of the landing page.
#[component]
pub fn HeroSection() -> Element {
    rsx! {
        header { class: "hero relative overflow-visible",
            { ambient_orbs() }
            div { class: "relative z-10",
                { eyebrow_and_title() }
                { headline_and_desc() }
                { cta_and_stats() }
            }
        }
    }
}

/// Two blurred background orbs giving depth.
fn ambient_orbs() -> Element {
    rsx! {
        div { class: "ambient-orb ambient-orb-primary \
            w-[500px] h-[500px] top-[-10%] left-[-15%]" }
        div { class: "ambient-orb ambient-orb-secondary \
            w-[400px] h-[400px] bottom-[20%] right-[-10%]" }
    }
}

/// Pill tagline and the split "PYCKX" logo.
fn eyebrow_and_title() -> Element {
    rsx! {
        div { class: "eyebrow bg-white/5 border \
            border-white/10 rounded-full px-4 py-1.5 \
            backdrop-blur-md",
            "\u{2726} La plateforme \u{00B7} des etudiants"
        }
        div { class: "split-wrap mb-6",
            span { class: "split-top", "PYCKX" }
            span { class: "split-bottom", "PYCKX" }
            div { dangerous_inner_html: SPLIT_LINE_SVG }
        }
    }
}

/// "Learn smarter. Play." headline and marketing paragraph.
fn headline_and_desc() -> Element {
    rsx! {
        h1 {
            "Learn "
            span { class: "c", "smarter." }
            " "
            span { class: "o", "Play." }
        }
        p { class: "desc",
            "Transformez n\u{2019}importe quel document en "
            strong { "quiz interactifs" }
            " en quelques secondes. 7 types de jeux, \
                correction IA, progression suivie \
                \u{2014} 100% gratuit."
        }
    }
}

/// Primary CTA, ghost link, and the key stats row.
fn cta_and_stats() -> Element {
    rsx! {
        HeroActions {
            CtaButton {
                label: "Commencer ",
                highlight: "gratuitement",
                to: "/login",
            }
            GhostButton {
                label: "Voir les jeux \u{2192}",
                target: GhostTarget::Anchor("#jeux"),
            }
        }
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
