//! Demo A — Neo-Brutalist (current style, refined)

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

/// Demo A page — neo-brutalist
pub fn DemoAPage() -> Element {
    rsx! {
        div { class: "da",
            // -- Navbar --
            nav { class: "da-nav",
                span { class: "da-logo", "PYCKX" }
                div { class: "da-nav-links",
                    span { class: "da-pill active", "Home" }
                    span { class: "da-pill", "Study" }
                    span { class: "da-pill", "Collection" }
                }
                div { class: "da-nav-actions",
                    button { class: "da-ghost", "D" }
                    button { class: "da-btn", "Logout" }
                }
            }
            // -- Content --
            main { class: "da-main",
                // Hero
                header { class: "da-hero",
                    h1 { class: "da-hero-title", "Study" }
                    p { class: "da-hero-sub",
                        "Choose a learning game"
                    }
                    div { class: "da-hero-actions",
                        button { class: "da-btn", "Generate" }
                        button { class: "da-btn-outline",
                            "Create"
                        }
                    }
                }
                // Game grid
                div { class: "da-grid",
                    for g in GAMES.iter() {
                        div { class: "da-card",
                            div { class: "da-card-icon",
                                "O"
                            }
                            h3 { class: "da-card-title",
                                "{g.name}"
                            }
                            p { class: "da-card-desc",
                                "{g.desc}"
                            }
                            div { class: "da-card-footer",
                                span { class: "da-badge",
                                    "{g.count} sets"
                                }
                                button { class: "da-btn-sm",
                                    "Play"
                                }
                            }
                        }
                    }
                }
                // Label
                p { class: "da-label",
                    "Style A — Neo-Brutalist"
                }
            }
        }
    }
}
