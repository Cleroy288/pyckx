// ** intello_home.rs **
// ==> Intello home — game grid with NavCard tiles

use crate::components::top_bar::HomeTopBar;
use crate::components::ui::card::{CardGrid, NavCard};
use crate::components::ui::hero_banner::HeroBanner;
use crate::components::ui::page_layout::PageLayout;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/pages/intello/intello_home.module.css"
);

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
#[component]
pub fn IntelloHomePage() -> impl IntoView {
    let nav = leptos_router::hooks::use_navigate();

    view! {
        <HomeTopBar />
        <PageLayout>
            <HeroBanner title="Intello">
                <p class=style::subtitle>
                    "Choose a learning game"
                </p>
            </HeroBanner>
            <CardGrid>
                {GAMES.iter().map(|g| {
                    let path = g.path;
                    let nav = nav.clone();
                    view! {
                        <NavCard
                            title=g.name
                            description=g.desc
                            icon=g.icon
                            on_click=Callback::new(
                                move |_| {
                                    nav(
                                        path,
                                        Default::default(),
                                    );
                                },
                            )
                        />
                    }
                }).collect_view()}
            </CardGrid>
        </PageLayout>
    }
}
