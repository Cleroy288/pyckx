//! Intello home — game grid with NavCard tiles

use crate::components::top_bar::HomeTopBar;
use crate::components::ui::card::{CardGrid, NavCard};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use dioxus::prelude::*;

/// Game link data
struct GameLink {
    name: &'static str,
    path: &'static str,
    icon: &'static str,
    desc: &'static str,
}

/// All available games
const GAMES: [GameLink; 8] = [
    GameLink {
        name: "QCM",
        path: "/intello/qcm",
        icon: "CheckCircle",
        desc: "Multiple choice questions",
    },
    GameLink {
        name: "Flashcards",
        path: "/intello/flashcards",
        icon: "Layers",
        desc: "Flip & memorize cards",
    },
    GameLink {
        name: "True / False",
        path: "/intello/true-false",
        icon: "Check",
        desc: "Statement challenges",
    },
    GameLink {
        name: "Open Questions",
        path: "/intello/open-questions",
        icon: "MessageSquare",
        desc: "Free-form answers",
    },
    GameLink {
        name: "Keywords",
        path: "/intello/keywords",
        icon: "Star",
        desc: "Spot the keywords",
    },
    GameLink {
        name: "Order Phrases",
        path: "/intello/order-phrases",
        icon: "List",
        desc: "Reorder the words",
    },
    GameLink {
        name: "Fill Blanks",
        path: "/intello/fill-blanks",
        icon: "HelpCircle",
        desc: "Complete the phrases",
    },
    GameLink {
        name: "Courses",
        path: "/intello/courses",
        icon: "Book",
        desc: "AI study sessions",
    },
];

/// Intello home page with game grid
pub fn IntelloHomePage() -> Element {
    let nav = navigator();

    rsx! {
        HomeTopBar {}
        PageLayout {
            HeroBanner {
                title: "Intello",
                p {
                    class: "relative z-[1] m-0 \
                        text-lg \
                        text-[var(--color-text-secondary)]",
                    "Choose a learning game"
                }
            }
            CardGrid {
                for g in GAMES.iter() {
                    NavCard {
                        title: g.name,
                        description: g.desc,
                        icon: g.icon,
                        on_click: {
                            let path = g.path;
                            let nav = nav.clone();
                            move |_: ()| {
                                nav.push(path);
                            }
                        },
                    }
                }
            }
        }
    }
}
