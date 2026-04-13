//! Home page — authenticated dashboard orchestration.
//!
//! Newspaper flow:
//!   1. [`HomePage`]  — guards the session, picks one state.
//!   2. [`HomeContent`] — composes hero + games + footer.
//!   3. Helpers: the games catalog and the loading banner.

use crate::auth::{use_auth_guard, AuthState};
use crate::home::deco::DecoStickers;
use crate::home::hero::{GamesBanner, HomeHero};
use crate::home::top_bar::HomeTopBar;
use crate::routes::Route;
use crate::ui::GameCard;
use dioxus::prelude::*;
use dioxus::router::Navigator;

/// Message shown while `GET /me` is in flight.
const CHECKING_MSG: &str = "Checking session...";

/// Message shown while the guard redirects to `/login`.
const REDIRECT_MSG: &str = "Redirecting to login...";

/// Centered message styling used by both loading states.
const MSG_CLS: &str = "flex justify-center \
    items-center min-h-[50vh] \
    text-[var(--muted-foreground)] text-lg";

/// Single entry for one playable game tile.
struct GameLink {
    tag: &'static str,
    name: &'static str,
    path: &'static str,
    desc: &'static str,
    featured: bool,
}

/// Every game surfaced on the dashboard.
const GAMES: [GameLink; 9] = [
    GameLink { tag: "01", name: "QCM", path: "/qcm",
        desc: "Questions a choix multiples", featured: true },
    GameLink { tag: "02", name: "Flashcards", path: "/flashcards",
        desc: "Flip & memorize cards", featured: false },
    GameLink { tag: "03", name: "True / False", path: "/true-false",
        desc: "Statement challenges", featured: false },
    GameLink { tag: "04", name: "Open Questions",
        path: "/open-questions",
        desc: "Free-form answers", featured: false },
    GameLink { tag: "05", name: "Keywords", path: "/keywords",
        desc: "Spot the keywords", featured: false },
    GameLink { tag: "06", name: "Order Phrases",
        path: "/order-phrases",
        desc: "Reorder the words", featured: false },
    GameLink { tag: "07", name: "Fill Blanks", path: "/fill-blanks",
        desc: "Complete the phrases", featured: false },
    GameLink { tag: "08", name: "Code", path: "/code/generate",
        desc: "Write & run code", featured: false },
    GameLink { tag: "09", name: "Courses", path: "/courses",
        desc: "AI study sessions", featured: false },
];

/// Home page — authenticated game hub.
pub fn HomePage() -> Element {
    let auth = use_auth_guard();
    rsx! {
        div { class: "home-page",
            HomeTopBar {}
            {render_body(&auth)}
        }
    }
}

/// Pick which body to render based on the session state.
fn render_body(auth: &AuthState) -> Element {
    if (auth.is_checking_session)() {
        status_message(CHECKING_MSG)
    } else if (auth.user)().is_some() {
        rsx! { HomeContent {} }
    } else {
        status_message(REDIRECT_MSG)
    }
}

/// Centered loading / redirect banner.
fn status_message(text: &'static str) -> Element {
    rsx! {
        div { class: "{MSG_CLS}", "{text}" }
    }
}

/// Dashboard body: hero, games section and footer link.
#[component]
fn HomeContent() -> Element {
    let nav = navigator();
    rsx! {
        DecoStickers {}
        main { class: "home-main",
            HomeHero {}
            section { class: "home-section",
                GamesBanner {}
                div { class: "games-grid home-games",
                    for game in GAMES.iter() {
                        {render_game_card(game, nav)}
                    }
                }
            }
            HomeFooter {}
        }
    }
}

/// Render one [`GameCard`] that navigates to its route.
fn render_game_card(game: &'static GameLink, nav: Navigator) -> Element {
    let path = game.path;
    rsx! {
        GameCard {
            key: "{game.tag}",
            tag: game.tag,
            name: game.name,
            desc: game.desc,
            featured: game.featured,
            on_click: move |_: ()| { nav.push(path); },
        }
    }
}

/// Minimal footer with a link back to the public site.
#[component]
fn HomeFooter() -> Element {
    rsx! {
        footer { class: "home-footer",
            Link {
                to: Route::Vitrine {},
                class: "home-footer-link",
                "\u{2190} Back to Pyckx.com"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_games_catalog_has_nine_entries() {
        assert_eq!(GAMES.len(), 9);
    }

    #[test]
    fn test_games_catalog_has_exactly_one_featured() {
        let featured = GAMES.iter().filter(|g| g.featured).count();
        assert_eq!(featured, 1);
    }

    #[test]
    fn test_games_tags_are_unique() {
        let mut tags: Vec<&str> = GAMES.iter().map(|g| g.tag).collect();
        tags.sort_unstable();
        let len_before = tags.len();
        tags.dedup();
        assert_eq!(tags.len(), len_before);
    }

    #[test]
    fn test_games_paths_start_with_slash() {
        assert!(GAMES.iter().all(|g| g.path.starts_with('/')));
    }

    #[test]
    fn test_qcm_is_the_featured_game() {
        let featured = GAMES.iter().find(|g| g.featured).unwrap();
        assert_eq!(featured.name, "QCM");
    }
}
