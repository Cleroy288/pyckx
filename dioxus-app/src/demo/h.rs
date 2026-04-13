//! Demo H — Quizlet-clean rendering with PYCKX split logo.
//!
//! Keeps the split logo identity but softens every other
//! surface. Composed of four sections: nav, hero, games, sets.

use crate::home::SPLIT_LINE_SVG;
use crate::ui::Icon;
use dioxus::prelude::*;

/// Interactive learning mode rendered in the game grid.
struct Game {
    name: &'static str,
    desc: &'static str,
    icon: &'static str,
}

const GAMES: [Game; 6] = [
    Game { name: "QCM", desc: "Multiple choice questions",
           icon: "CheckCircle" },
    Game { name: "Flashcards", desc: "Flip & memorize cards",
           icon: "Layers" },
    Game { name: "True / False", desc: "Statement challenges",
           icon: "Check" },
    Game { name: "Open Questions", desc: "Free-form answers",
           icon: "MessageSquare" },
    Game { name: "Keywords", desc: "Spot the keywords",
           icon: "Star" },
    Game { name: "Courses", desc: "AI study sessions",
           icon: "Book" },
];

/// User-created question set rendered in the set grid.
struct FakeSet {
    name: &'static str,
    level: &'static str,
    count: u32,
}

const SETS: [FakeSet; 4] = [
    FakeSet { name: "Biology Chapter 5", level: "medium", count: 15 },
    FakeSet { name: "French Vocabulary", level: "easy", count: 30 },
    FakeSet { name: "Quantum Mechanics", level: "hard", count: 8 },
    FakeSet { name: "World History", level: "medium", count: 20 },
];

/// Demo H page — Quizlet-clean with split PYCKX logo.
pub fn DemoHPage() -> Element {
    rsx! {
        div { class: "dh",
            {nav_bar()}
            main { class: "dh-main",
                {hero()}
                {section_header("Learning modes", "6 available")}
                {game_grid()}
                {section_header("Your sets", "4 created")}
                {set_grid()}
                p { class: "dh-label",
                    "Style H — Quizlet-clean + PYCKX identity"
                }
            }
        }
    }
}

/// Top navigation bar with the split logo.
fn nav_bar() -> Element {
    rsx! {
        nav { class: "dh-nav",
            {nav_logo()}
            div { class: "dh-nav-links",
                span { class: "dh-pill", "Home" }
                span { class: "dh-pill active", "Study" }
                span { class: "dh-pill", "Collection" }
            }
            div { class: "dh-nav-actions",
                button { class: "dh-icon-btn",
                    Icon { icon_name: "Moon".to_string() }
                }
                button { class: "dh-btn-primary", "Logout" }
            }
        }
    }
}

/// PYCKX split logo link that routes to `/demo/h`.
fn nav_logo() -> Element {
    rsx! {
        Link {
            to: "/demo/h",
            class: "dh-logo no-underline",
            div { class: "split-wrap-sm",
                span { class: "split-top", "PYCKX" }
                span { class: "split-bottom", "PYCKX" }
                div { dangerous_inner_html: SPLIT_LINE_SVG }
            }
        }
    }
}

/// Hero header with split `Study` title and call-to-action.
fn hero() -> Element {
    rsx! {
        header { class: "dh-hero",
            div { class: "dh-hero-split",
                div { class: "split-wrap",
                    span { class: "split-top", "Study" }
                    span { class: "split-bottom", "Study" }
                    div { dangerous_inner_html: SPLIT_LINE_SVG }
                }
            }
            p { class: "dh-hero-sub", "Choose a learning game" }
            div { class: "dh-hero-actions",
                button { class: "dh-btn-primary",
                    "Generate with AI"
                }
                button { class: "dh-btn-outline",
                    "Create manually"
                }
            }
        }
    }
}

/// Small section header with a title label and count badge.
fn section_header(label: &str, count: &str) -> Element {
    rsx! {
        div { class: "dh-section",
            span { class: "dh-section-label", "{label}" }
            span { class: "dh-section-count", "{count}" }
        }
    }
}

/// Grid of learning-mode cards built from `GAMES`.
fn game_grid() -> Element {
    rsx! {
        div { class: "dh-game-grid",
            for g in GAMES.iter() {
                button { class: "dh-game-card",
                    div { class: "dh-game-icon",
                        Icon { icon_name: g.icon.to_string() }
                    }
                    span { class: "dh-game-name", "{g.name}" }
                    span { class: "dh-game-desc", "{g.desc}" }
                }
            }
        }
    }
}

/// Grid of user-set cards built from `SETS`.
fn set_grid() -> Element {
    rsx! {
        div { class: "dh-set-grid",
            for s in SETS.iter() {
                {set_card(s)}
            }
        }
    }
}

/// Single set card with header, meta, and action buttons.
fn set_card(s: &FakeSet) -> Element {
    rsx! {
        div { class: "dh-set-card",
            div { class: "dh-set-header",
                h3 { class: "dh-set-name", "{s.name}" }
                span { class: "dh-level dh-level-{s.level}",
                    "{s.level}"
                }
            }
            span { class: "dh-set-meta", "{s.count} questions" }
            div { class: "dh-set-actions",
                button { class: "dh-action-play", "Play" }
                button { class: "dh-action-del", "Delete" }
            }
        }
    }
}
