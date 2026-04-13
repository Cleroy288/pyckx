//! Hero + section banners for the home dashboard.
//!
//! Purely presentational: no state, no data fetching.

use crate::home::logo::SPLIT_LINE_SVG;
use dioxus::prelude::*;

/// Total number of games shown in the games banner.
const GAMES_COUNT: &str = "9";

/// Time needed for a PDF to become a quiz.
const PDF_TO_QUIZ: &str = "10s";

/// Infinity symbol used for the "free" stat.
const INFINITY: &str = "\u{221E}";

/// Neo-brutalist hero with headline + stats.
#[component]
pub fn HomeHero() -> Element {
    rsx! {
        header { class: "home-hero",
            div { class: "home-hero-card",
                div { class: "home-eyebrow", "\u{2726} Dashboard" }
                HeroWordmark {}
                h1 { class: "home-headline",
                    "Let\u{2019}s "
                    span { class: "hi-accent", "play." }
                }
                p { class: "home-desc",
                    "Choose a game, upload your notes, \
                        and start learning."
                }
                HeroStats {}
            }
        }
    }
}

/// Split "PYCKX" wordmark rendered inside the hero card.
#[component]
fn HeroWordmark() -> Element {
    rsx! {
        div { class: "split-wrap home-split",
            span { class: "split-top", "PYCKX" }
            span { class: "split-bottom", "PYCKX" }
            div { dangerous_inner_html: SPLIT_LINE_SVG }
        }
    }
}

/// Three-item stats row shown at the bottom of the hero.
#[component]
fn HeroStats() -> Element {
    rsx! {
        div { class: "home-stats",
            StatItem { value: GAMES_COUNT, label: "Games" }
            div { class: "home-stat-sep" }
            StatItem { value: PDF_TO_QUIZ, label: "PDF \u{2192} Quiz" }
            div { class: "home-stat-sep" }
            StatItem { value: INFINITY, label: "Free" }
        }
    }
}

/// Single value/label stat pill used in the hero row.
#[component]
pub fn StatItem(
    value: &'static str,
    label: &'static str,
) -> Element {
    rsx! {
        div { class: "home-stat",
            span { class: "home-stat-val", "{value}" }
            span { class: "home-stat-lbl", "{label}" }
        }
    }
}

/// Diagonal-stripe banner introducing the games section.
#[component]
pub fn GamesBanner() -> Element {
    rsx! {
        div { class: "games-banner-card",
            div { class: "games-banner-stripes" }
            div { class: "games-banner-body",
                span { class: "games-banner-num", "{GAMES_COUNT}" }
                h2 { class: "games-banner-title",
                    "learning "
                    span { class: "games-banner-hi", "games" }
                }
            }
        }
    }
}
