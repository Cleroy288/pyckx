//! Demo C — Hybrid (brutalist accents, soft cards)

use dioxus::prelude::*;

/// Mock game data
struct Game {
    name: &'static str,
    desc: &'static str,
    count: u32,
}

const GAMES: [Game; 6] = [
    Game { name: "QCM", desc: "Multiple choice", count: 12 },
    Game { name: "Flashcards", desc: "Flip & memorize", count: 8 },
    Game { name: "True / False", desc: "Statement challenges", count: 5 },
    Game { name: "Open Questions", desc: "Free-form answers", count: 3 },
    Game { name: "Keywords", desc: "Spot the keywords", count: 7 },
    Game { name: "Courses", desc: "AI study sessions", count: 2 },
];

/// Demo C page — hybrid style
pub fn DemoCPage() -> Element {
    rsx! {
        div { class: "dc",
            // -- Navbar --
            nav { class: "dc-nav",
                span { class: "dc-logo", "PYCKX" }
                div { class: "dc-nav-links",
                    span { class: "dc-pill active", "Home" }
                    span { class: "dc-pill", "Study" }
                    span { class: "dc-pill", "Collection" }
                }
                div { class: "dc-nav-actions",
                    button { class: "dc-ghost", "D" }
                    button { class: "dc-btn", "Logout" }
                }
            }
            // -- Content --
            main { class: "dc-main",
                // Hero (brutalist accent)
                header { class: "dc-hero",
                    h1 { class: "dc-hero-title", "Study" }
                    p { class: "dc-hero-sub",
                        "Choose a learning game"
                    }
                    div { class: "dc-hero-actions",
                        button { class: "dc-btn-accent",
                            "Generate"
                        }
                        button { class: "dc-btn-outline",
                            "Create"
                        }
                    }
                }
                // Game grid (soft cards)
                div { class: "dc-grid",
                    for g in GAMES.iter() {
                        div { class: "dc-card",
                            div { class: "dc-card-icon",
                                "O"
                            }
                            h3 { class: "dc-card-title",
                                "{g.name}"
                            }
                            p { class: "dc-card-desc",
                                "{g.desc}"
                            }
                            div { class: "dc-card-footer",
                                span { class: "dc-badge",
                                    "{g.count} sets"
                                }
                                button { class: "dc-btn-sm",
                                    "Play"
                                }
                            }
                        }
                    }
                }
                // Label
                p { class: "dc-label",
                    "Style C — Hybrid"
                }
            }
        }
    }
}
