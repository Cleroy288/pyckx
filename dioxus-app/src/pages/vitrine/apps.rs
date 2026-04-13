//! AppsSection — game types showcase grid

use crate::components::ui::card::GameCard;
use crate::components::vitrine::SectionHeader;
use dioxus::prelude::*;

/// Game type for the feature grid
struct GameType {
    tag: &'static str,
    name: &'static str,
    desc: &'static str,
    featured: bool,
}

/// All 7 game types
const GAMES: [GameType; 7] = [
    GameType {
        tag: "01",
        name: "QCM",
        desc: "Questions a choix multiples \
            generees par IA",
        featured: true,
    },
    GameType {
        tag: "02",
        name: "Flashcards",
        desc: "Memorisation par repetition \
            espacee",
        featured: false,
    },
    GameType {
        tag: "03",
        name: "Vrai / Faux",
        desc: "Validation rapide \
            des concepts cles",
        featured: false,
    },
    GameType {
        tag: "04",
        name: "Questions ouvertes",
        desc: "Reponses libres corrigees \
            par IA",
        featured: false,
    },
    GameType {
        tag: "05",
        name: "Mots-cles",
        desc: "Identifiez les termes \
            essentiels du cours",
        featured: false,
    },
    GameType {
        tag: "06",
        name: "Ordre",
        desc: "Remettez les etapes \
            dans le bon ordre",
        featured: false,
    },
    GameType {
        tag: "07",
        name: "Texte a trous",
        desc: "Completez les passages \
            manquants",
        featured: false,
    },
];

/// Game types showcase section
#[component]
pub fn AppsSection() -> Element {
    rsx! {
        section {
            class: "vt-section",
            id: "jeux",
            SectionHeader {
                title: rsx! {
                    "7 formats" br {} "de quiz."
                },
                description: "Un seul document. \
                    Sept facons de reviser.",
            }
            div { class: "games-grid",
                for game in GAMES.iter() {
                    GameCard {
                        key: "{game.tag}",
                        tag: game.tag,
                        name: game.name,
                        desc: game.desc,
                        featured: game.featured,
                    }
                }
            }
        }
    }
}
