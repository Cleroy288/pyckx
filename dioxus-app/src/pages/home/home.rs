//! Home Page — neo-brutalist game hub

use crate::components::logo::SPLIT_LINE_SVG;
use crate::components::top_bar::HomeTopBar;
use crate::components::ui::card::GameCard;
use crate::routes::Route;
use crate::state::use_auth_guard;
use dioxus::prelude::*;

/// Game link data
struct GameLink {
    tag: &'static str,
    name: &'static str,
    path: &'static str,
    desc: &'static str,
    featured: bool,
}

/// All available games
const GAMES: [GameLink; 9] = [
    GameLink {
        tag: "01",
        name: "QCM",
        path: "/qcm",
        desc: "Questions a choix multiples",
        featured: true,
    },
    GameLink {
        tag: "02",
        name: "Flashcards",
        path: "/flashcards",
        desc: "Flip & memorize cards",
        featured: false,
    },
    GameLink {
        tag: "03",
        name: "True / False",
        path: "/true-false",
        desc: "Statement challenges",
        featured: false,
    },
    GameLink {
        tag: "04",
        name: "Open Questions",
        path: "/open-questions",
        desc: "Free-form answers",
        featured: false,
    },
    GameLink {
        tag: "05",
        name: "Keywords",
        path: "/keywords",
        desc: "Spot the keywords",
        featured: false,
    },
    GameLink {
        tag: "06",
        name: "Order Phrases",
        path: "/order-phrases",
        desc: "Reorder the words",
        featured: false,
    },
    GameLink {
        tag: "07",
        name: "Fill Blanks",
        path: "/fill-blanks",
        desc: "Complete the phrases",
        featured: false,
    },
    GameLink {
        tag: "08",
        name: "Code",
        path: "/code/generate",
        desc: "Write & run code",
        featured: false,
    },
    GameLink {
        tag: "09",
        name: "Courses",
        path: "/courses",
        desc: "AI study sessions",
        featured: false,
    },
];

/// Loading / redirect message
const MSG_CLS: &str = "flex justify-center \
    items-center min-h-[50vh] \
    text-[var(--muted-foreground)] text-lg";

/// Home page — authenticated game hub
pub fn HomePage() -> Element {
    let auth = use_auth_guard();

    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            if (auth.is_checking_session)() {
                div { class: "{MSG_CLS}",
                    "Checking session..."
                }
            } else if (auth.user)().is_some() {
                HomeContent {}
            } else {
                div { class: "{MSG_CLS}",
                    "Redirecting to login..."
                }
            }
        }
    }
}

/// Main home content (hero + grid + theme)
#[component]
fn HomeContent() -> Element {
    let nav = navigator();

    rsx! {
        DecoStickers {}
        main { class: "home-main",
            HomeHero {}
            div { class: "home-section",
                GamesBanner {}
                div { class: "games-grid home-games",
                    for g in GAMES.iter() {
                        GameCard {
                            key: "{g.tag}",
                            tag: g.tag,
                            name: g.name,
                            desc: g.desc,
                            featured: g.featured,
                            on_click: {
                                let p = g.path;
                                move |_: ()| {
                                    nav.push(p);
                                }
                            },
                        }
                    }
                }
            }
            HomeFooter {}
        }
    }
}

use super::deco::DecoStickers;

/// Neo-brutalist hero with headline + stats
#[component]
fn HomeHero() -> Element {
    rsx! {
        header { class: "home-hero",
            // White card floating over the pattern
            div { class: "home-hero-card",
                div { class: "home-eyebrow",
                    "\u{2726} Dashboard"
                }
                div { class: "split-wrap home-split",
                    span { class: "split-top", "PYCKX" }
                    span {
                        class: "split-bottom",
                        "PYCKX"
                    }
                    div {
                        dangerous_inner_html:
                            SPLIT_LINE_SVG,
                    }
                }
                h1 { class: "home-headline",
                    "Let\u{2019}s "
                    span { class: "hi-accent", "play." }
                }
                p { class: "home-desc",
                    "Choose a game, upload your notes, \
                        and start learning."
                }
                div { class: "home-stats",
                    StatItem {
                        value: "9",
                        label: "Games",
                    }
                    div { class: "home-stat-sep" }
                    StatItem {
                        value: "10s",
                        label: "PDF \u{2192} Quiz",
                    }
                    div { class: "home-stat-sep" }
                    StatItem {
                        value: "\u{221E}",
                        label: "Free",
                    }
                }
            }
        }
    }
}

/// Single stat pill
#[component]
fn StatItem(
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

/// Section header with green accent line
#[component]
fn SectionHead(title: &'static str) -> Element {
    rsx! {
        div { class: "home-section-hd",
            span { class: "home-section-line" }
            h2 { "{title}" }
        }
    }
}

/// Diagonal-stripe banner — games section header
#[component]
fn GamesBanner() -> Element {
    rsx! {
        div { class: "games-banner-card",
            div { class: "games-banner-stripes" }
            div { class: "games-banner-body",
                span { class: "games-banner-num", "9" }
                h2 { class: "games-banner-title",
                    "learning "
                    span { class: "games-banner-hi",
                        "games"
                    }
                }
            }
        }
    }
}

/// Minimal footer with link back to vitrine
#[component]
fn HomeFooter() -> Element {
    rsx! {
        footer { class: "home-footer",
            Link {
                to: Route::Vitrine {},
                class: "home-footer-link",
                "\u{2190} Back to Pyckx.com"
            }
        }
    }
}
