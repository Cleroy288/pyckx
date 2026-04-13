//! Demo D — Clean + Primary border accent

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

pub fn DemoDPage() -> Element {
    rsx! {
        div { class: "dd",
            nav { class: "dd-nav",
                span { class: "dd-logo", "PYCKX" }
                div { class: "dd-nav-links",
                    span { class: "dd-pill active", "Home" }
                    span { class: "dd-pill", "Study" }
                    span { class: "dd-pill", "Collection" }
                }
                div { class: "dd-nav-actions",
                    button { class: "dd-ghost", "D" }
                    button { class: "dd-btn", "Logout" }
                }
            }
            main { class: "dd-main",
                header { class: "dd-hero",
                    h1 { class: "dd-hero-title", "Study" }
                    p { class: "dd-hero-sub", "Choose a learning game" }
                    div { class: "dd-hero-actions",
                        button { class: "dd-btn", "Generate" }
                        button { class: "dd-btn-sec", "Create" }
                    }
                }
                div { class: "dd-grid",
                    for g in GAMES.iter() {
                        div { class: "dd-card",
                            div { class: "dd-card-icon", "O" }
                            h3 { class: "dd-card-title", "{g.name}" }
                            p { class: "dd-card-desc", "{g.desc}" }
                            div { class: "dd-card-footer",
                                span { class: "dd-badge", "{g.count} sets" }
                                button { class: "dd-btn-sm", "Play" }
                            }
                        }
                    }
                }
                p { class: "dd-label",
                    "Style D — Clean + Primary Border"
                }
            }
        }
    }
}
