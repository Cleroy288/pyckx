//! Demo F — Clean + Paper cutout (strong elevation)

use dioxus::prelude::*;

struct Game { name: &'static str, desc: &'static str, count: u32 }

const GAMES: [Game; 6] = [
    Game { name: "QCM", desc: "Multiple choice", count: 12 },
    Game { name: "Flashcards", desc: "Flip & memorize", count: 8 },
    Game { name: "True / False", desc: "Statement challenges", count: 5 },
    Game { name: "Open Questions", desc: "Free-form answers", count: 3 },
    Game { name: "Keywords", desc: "Spot the keywords", count: 7 },
    Game { name: "Courses", desc: "AI study sessions", count: 2 },
];

pub fn DemoFPage() -> Element {
    rsx! {
        div { class: "df",
            nav { class: "df-nav",
                span { class: "df-logo", "PYCKX" }
                div { class: "df-nav-links",
                    span { class: "df-pill active", "Home" }
                    span { class: "df-pill", "Study" }
                    span { class: "df-pill", "Collection" }
                }
                div { class: "df-nav-actions",
                    button { class: "df-ghost", "D" }
                    button { class: "df-btn", "Logout" }
                }
            }
            main { class: "df-main",
                header { class: "df-hero",
                    h1 { class: "df-hero-title", "Study" }
                    p { class: "df-hero-sub", "Choose a learning game" }
                    div { class: "df-hero-actions",
                        button { class: "df-btn", "Generate" }
                        button { class: "df-btn-sec", "Create" }
                    }
                }
                div { class: "df-grid",
                    for g in GAMES.iter() {
                        div { class: "df-card",
                            div { class: "df-card-icon", "O" }
                            h3 { class: "df-card-title", "{g.name}" }
                            p { class: "df-card-desc", "{g.desc}" }
                            div { class: "df-card-footer",
                                span { class: "df-badge", "{g.count} sets" }
                                button { class: "df-btn-sm", "Play" }
                            }
                        }
                    }
                }
                p { class: "df-label",
                    "Style F — Paper Cutout (elevated)"
                }
            }
        }
    }
}
