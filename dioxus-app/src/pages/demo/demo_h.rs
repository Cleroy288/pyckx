//! Demo H — Current app restyled Quizlet-clean
//! Logo découpé PYCKX conservé, tout le reste adouci.

use crate::components::logo::SPLIT_LINE_SVG;
use crate::components::ui::icon::Icon;
use dioxus::prelude::*;

struct Game {
    name: &'static str,
    desc: &'static str,
    icon: &'static str,
}

const GAMES: [Game; 6] = [
    Game { name: "QCM", desc: "Multiple choice questions", icon: "CheckCircle" },
    Game { name: "Flashcards", desc: "Flip & memorize cards", icon: "Layers" },
    Game { name: "True / False", desc: "Statement challenges", icon: "Check" },
    Game { name: "Open Questions", desc: "Free-form answers", icon: "MessageSquare" },
    Game { name: "Keywords", desc: "Spot the keywords", icon: "Star" },
    Game { name: "Courses", desc: "AI study sessions", icon: "Book" },
];

/// Fake set for item card demo
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

pub fn DemoHPage() -> Element {
    rsx! {
        div { class: "dh",
            // ── Navbar ──
            nav { class: "dh-nav",
                // Logo découpé (conservé)
                Link {
                    to: "/demo/h",
                    class: "dh-logo no-underline",
                    div { class: "split-wrap-sm",
                        span {
                            class: "split-top",
                            "PYCKX"
                        }
                        span {
                            class: "split-bottom",
                            "PYCKX"
                        }
                        div {
                            dangerous_inner_html:
                                SPLIT_LINE_SVG,
                        }
                    }
                }
                div { class: "dh-nav-links",
                    span { class: "dh-pill", "Home" }
                    span { class: "dh-pill active",
                        "Study"
                    }
                    span { class: "dh-pill",
                        "Collection"
                    }
                }
                div { class: "dh-nav-actions",
                    button { class: "dh-icon-btn",
                        Icon {
                            icon_name: "Moon".to_string(),
                        }
                    }
                    button { class: "dh-btn-primary",
                        "Logout"
                    }
                }
            }

            // ── Content ──
            main { class: "dh-main",
                // Hero
                header { class: "dh-hero",
                    div { class: "dh-hero-split",
                        div { class: "split-wrap",
                            span {
                                class: "split-top",
                                "Study"
                            }
                            span {
                                class: "split-bottom",
                                "Study"
                            }
                            div {
                                dangerous_inner_html:
                                    SPLIT_LINE_SVG,
                            }
                        }
                    }
                    p { class: "dh-hero-sub",
                        "Choose a learning game"
                    }
                    div { class: "dh-hero-actions",
                        button { class: "dh-btn-primary",
                            "Generate with AI"
                        }
                        button { class: "dh-btn-outline",
                            "Create manually"
                        }
                    }
                }

                // Game grid (NavCard style)
                div { class: "dh-section",
                    span { class: "dh-section-label",
                        "Learning modes"
                    }
                    span { class: "dh-section-count",
                        "6 available"
                    }
                }
                div { class: "dh-game-grid",
                    for g in GAMES.iter() {
                        button { class: "dh-game-card",
                            div { class: "dh-game-icon",
                                Icon {
                                    icon_name: g.icon
                                        .to_string(),
                                }
                            }
                            span {
                                class: "dh-game-name",
                                "{g.name}"
                            }
                            span {
                                class: "dh-game-desc",
                                "{g.desc}"
                            }
                        }
                    }
                }

                // Item cards (QCM sets style)
                div { class: "dh-section",
                    span { class: "dh-section-label",
                        "Your sets"
                    }
                    span { class: "dh-section-count",
                        "4 created"
                    }
                }
                div { class: "dh-set-grid",
                    for s in SETS.iter() {
                        div { class: "dh-set-card",
                            div {
                                class: "dh-set-header",
                                h3 { class: "dh-set-name",
                                    "{s.name}"
                                }
                                span {
                                    class: "dh-level \
                                        dh-level-{s.level}",
                                    "{s.level}"
                                }
                            }
                            span { class: "dh-set-meta",
                                "{s.count} questions"
                            }
                            div { class: "dh-set-actions",
                                button {
                                    class: "dh-action-play",
                                    "Play"
                                }
                                button {
                                    class: "dh-action-del",
                                    "Delete"
                                }
                            }
                        }
                    }
                }

                p { class: "dh-label",
                    "Style H — Quizlet-clean \
                     + PYCKX identity"
                }
            }
        }
    }
}
