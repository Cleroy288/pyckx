//! Demo E — Clean + Top accent stripe (cutout feel)

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

pub fn DemoEPage() -> Element {
    rsx! {
        div { class: "de",
            nav { class: "de-nav",
                span { class: "de-logo", "PYCKX" }
                div { class: "de-nav-links",
                    span { class: "de-pill active", "Home" }
                    span { class: "de-pill", "Study" }
                    span { class: "de-pill", "Collection" }
                }
                div { class: "de-nav-actions",
                    button { class: "de-ghost", "D" }
                    button { class: "de-btn", "Logout" }
                }
            }
            main { class: "de-main",
                header { class: "de-hero",
                    h1 { class: "de-hero-title", "Study" }
                    p { class: "de-hero-sub", "Choose a learning game" }
                    div { class: "de-hero-actions",
                        button { class: "de-btn", "Generate" }
                        button { class: "de-btn-sec", "Create" }
                    }
                }
                div { class: "de-grid",
                    for g in GAMES.iter() {
                        div { class: "de-card",
                            div { class: "de-card-icon", "O" }
                            h3 { class: "de-card-title", "{g.name}" }
                            p { class: "de-card-desc", "{g.desc}" }
                            div { class: "de-card-footer",
                                span { class: "de-badge", "{g.count} sets" }
                                button { class: "de-btn-sm", "Play" }
                            }
                        }
                    }
                }
                p { class: "de-label",
                    "Style E — Clean + Top Accent Stripe"
                }
            }
        }
    }
}
