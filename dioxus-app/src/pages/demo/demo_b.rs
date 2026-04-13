//! Demo B — Clean / Soft (Quizlet-inspired)

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

/// Demo B page — clean/soft Quizlet style
pub fn DemoBPage() -> Element {
    rsx! {
        div { class: "db",
            // -- Navbar --
            nav { class: "db-nav",
                span { class: "db-logo", "PYCKX" }
                div { class: "db-nav-links",
                    span { class: "db-pill active", "Home" }
                    span { class: "db-pill", "Study" }
                    span { class: "db-pill", "Collection" }
                }
                div { class: "db-nav-actions",
                    button { class: "db-ghost", "D" }
                    button { class: "db-btn", "Logout" }
                }
            }
            // -- Content --
            main { class: "db-main",
                // Hero
                header { class: "db-hero",
                    h1 { class: "db-hero-title", "Study" }
                    p { class: "db-hero-sub",
                        "Choose a learning game"
                    }
                    div { class: "db-hero-actions",
                        button { class: "db-btn", "Generate" }
                        button { class: "db-btn-sec",
                            "Create"
                        }
                    }
                }
                // Game grid
                div { class: "db-grid",
                    for g in GAMES.iter() {
                        div { class: "db-card",
                            div { class: "db-card-icon",
                                "O"
                            }
                            h3 { class: "db-card-title",
                                "{g.name}"
                            }
                            p { class: "db-card-desc",
                                "{g.desc}"
                            }
                            div { class: "db-card-footer",
                                span { class: "db-badge",
                                    "{g.count} sets"
                                }
                                button { class: "db-btn-sm",
                                    "Play"
                                }
                            }
                        }
                    }
                }
                // Label
                p { class: "db-label",
                    "Style B — Clean / Soft (Quizlet)"
                }
            }
        }
    }
}
