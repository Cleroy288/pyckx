//! IntelloProductPage — public marketing page

use dioxus::prelude::*;

use super::nav::VitrineNav;

/// Game type for the feature grid
struct GameType {
    emoji: &'static str,
    name: &'static str,
    desc: &'static str,
}

/// All Intello game types
const GAMES: [GameType; 7] = [
    GameType {
        emoji: "\u{1F4DD}",
        name: "QCM",
        desc: "Questions a choix multiples",
    },
    GameType {
        emoji: "\u{1F0CF}",
        name: "Flashcards",
        desc: "Memorisation par repetition",
    },
    GameType {
        emoji: "\u{2705}",
        name: "Vrai / Faux",
        desc: "Validation rapide des concepts",
    },
    GameType {
        emoji: "\u{1F4AC}",
        name: "Questions ouvertes",
        desc: "Reponses longues corrigees par IA",
    },
    GameType {
        emoji: "\u{1F511}",
        name: "Mots-cles",
        desc: "Retrouvez les termes essentiels",
    },
    GameType {
        emoji: "\u{1F522}",
        name: "Ordre de phrases",
        desc: "Remettez les etapes dans l\u{2019}ordre",
    },
    GameType {
        emoji: "\u{270F}\u{FE0F}",
        name: "Texte a trous",
        desc: "Completez les passages manquants",
    },
];

/// How-it-works steps
const STEPS: [(&str, &str); 3] = [
    (
        "\u{1F4C4}",
        "Uploadez votre PDF, Word ou TXT",
    ),
    (
        "\u{1F9E0}",
        "L\u{2019}IA genere quiz et cours en 10s",
    ),
    (
        "\u{1F3AF}",
        "Jouez, apprenez, progressez",
    ),
];

/// Intello product marketing page
#[component]
pub fn IntelloProductPage() -> Element {
    rsx! {
        div { class: "vitrine-page",
            VitrineNav {}
            ProductHero {}
            GameTypesGrid {}
            HowItWorks {}
            ProductCta {}
        }
    }
}

/// Hero section with Intello branding
#[component]
fn ProductHero() -> Element {
    rsx! {
        header { class: "hero",
            div { class: "eyebrow",
                "\u{1F9E0} Intello \u{00B7} \
                    Assistant d\u{2019}apprentissage"
            }
            h1 {
                "Apprenez "
                span { class: "c", "plus vite" }
                " avec "
                span { class: "o", "l\u{2019}IA." }
            }
            p { class: "desc",
                "Transformez n\u{2019}importe quel \
                    document en "
                strong { "quiz interactifs" }
                " et "
                strong { "cours structures" }
                " en quelques secondes. \
                    7 types de jeux, correction \
                    automatique, progression suivie."
            }
            div { class: "hero-actions",
                Link { to: "/login",
                    span { class: "btn-ink",
                        "Essayer "
                        span { class: "cta-hi",
                            "gratuitement"
                        }
                    }
                }
                a {
                    href: "#jeux",
                    class: "btn-ghost",
                    "Voir les jeux \u{2192}"
                }
            }
        }
    }
}

/// Grid of 7 game types
#[component]
fn GameTypesGrid() -> Element {
    rsx! {
        section {
            class: "vt-section",
            id: "jeux",
            div { class: "section-hd",
                h2 { "7 types" br {} "de jeux." }
                p {
                    "Chaque document genere \
                        automatiquement 7 formats \
                        de quiz differents."
                }
            }
            div { class: "games-grid",
                for game in GAMES.iter() {
                    div { class: "game-card",
                        span {
                            class: "game-emoji",
                            "{game.emoji}"
                        }
                        h3 { "{game.name}" }
                        p { "{game.desc}" }
                    }
                }
            }
        }
    }
}

/// How it works — 3 steps
#[component]
fn HowItWorks() -> Element {
    rsx! {
        section { class: "vt-section",
            div { class: "section-hd",
                h2 { "Comment" br {} "ca marche." }
                p {
                    "De votre document a un quiz \
                        complet en 3 etapes simples."
                }
            }
            div { class: "steps-row",
                for (i, (emoji, text)) in
                    STEPS.iter().enumerate()
                {
                    div { class: "step-card",
                        span { class: "step-num",
                            "{i + 1}"
                        }
                        span {
                            class: "step-emoji",
                            "{emoji}"
                        }
                        p { "{text}" }
                    }
                    if i < 2 {
                        span {
                            class: "step-arrow",
                            "\u{2192}"
                        }
                    }
                }
            }
        }
    }
}

/// Bottom CTA
#[component]
fn ProductCta() -> Element {
    rsx! {
        section { class: "pricing-wrap",
            h2 { "Pret a apprendre ?" }
            div { class: "hero-actions",
                Link { to: "/login",
                    span { class: "btn-ink",
                        "Commencer "
                        span { class: "cta-hi",
                            "maintenant \u{2192}"
                        }
                    }
                }
                Link { to: "/",
                    span { class: "btn-ghost",
                        "Retour a l\u{2019}accueil"
                    }
                }
            }
        }
    }
}
