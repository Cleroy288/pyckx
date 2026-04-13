//! Demo G — Brutalist Light (heavy accents, soft repeats)

use dioxus::prelude::*;

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

pub fn DemoGPage() -> Element {
    rsx! {
        div { class: "dg",
            nav { class: "dg-nav",
                span { class: "dg-logo", "PYCKX" }
                div { class: "dg-nav-links",
                    span { class: "dg-pill active",
                        "Home"
                    }
                    span { class: "dg-pill", "Study" }
                    span { class: "dg-pill",
                        "Collection"
                    }
                }
                div { class: "dg-nav-actions",
                    button { class: "dg-icon-btn", "D" }
                    button { class: "dg-btn-primary",
                        "Logout"
                    }
                }
            }
            main { class: "dg-main",
                // Hero — brutalist (signature)
                header { class: "dg-hero",
                    h1 { class: "dg-hero-title",
                        "Study"
                    }
                    p { class: "dg-hero-sub",
                        "Choose a learning game"
                    }
                    div { class: "dg-hero-actions",
                        button { class: "dg-btn-primary",
                            "Generate"
                        }
                        button { class: "dg-btn-ghost",
                            "Create"
                        }
                    }
                }
                // Cards — soft
                div { class: "dg-grid",
                    for g in GAMES.iter() {
                        div { class: "dg-card",
                            div { class: "dg-card-icon",
                                "O"
                            }
                            h3 { class: "dg-card-title",
                                "{g.name}"
                            }
                            p { class: "dg-card-desc",
                                "{g.desc}"
                            }
                            div { class: "dg-card-footer",
                                span { class: "dg-badge",
                                    "{g.count} sets"
                                }
                                button {
                                    class: "dg-btn-sm",
                                    "Play"
                                }
                            }
                        }
                    }
                }
                p { class: "dg-label",
                    "Style G — Brutalist Light"
                }
            }
        }
    }
}
