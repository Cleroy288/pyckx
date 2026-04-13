//! Shared scaffolding for demo A-G pages.
//!
//! Each demo is a visual variant built on the same layout:
//! nav, hero, grid of game cards, footer label. Only CSS
//! class prefixes differ. This module keeps the layout in
//! one place; demos just pass their prefix and label.

use dioxus::prelude::*;

/// Mock game entry rendered in the card grid.
pub struct Game {
    pub name: &'static str,
    pub desc: &'static str,
    pub count: u32,
}

/// Shared sample data used across every demo variant.
pub const GAMES: [Game; 6] = [
    Game { name: "QCM", desc: "Multiple choice", count: 12 },
    Game { name: "Flashcards", desc: "Flip & memorize", count: 8 },
    Game { name: "True / False", desc: "Statement challenges", count: 5 },
    Game { name: "Open Questions", desc: "Free-form answers", count: 3 },
    Game { name: "Keywords", desc: "Spot the keywords", count: 7 },
    Game { name: "Courses", desc: "AI study sessions", count: 2 },
];

/// Button labels for the hero's primary/secondary actions.
pub struct HeroButtons {
    pub primary: &'static str,
    pub secondary: &'static str,
}

/// CSS class suffixes that differ between variants.
pub struct Variant {
    /// Prefix applied to every class (e.g. "da", "db").
    pub prefix: &'static str,
    /// Class used for the hero's primary action button.
    pub hero_primary_class: &'static str,
    /// Class used for the hero's secondary action button.
    pub hero_secondary_class: &'static str,
    /// Footer label shown at the bottom of the page.
    pub label: &'static str,
}

impl Variant {
    /// Builds a class name by joining the prefix and suffix.
    fn cls(&self, suffix: &str) -> String {
        format!("{}-{}", self.prefix, suffix)
    }
}

/// Renders the full demo page (nav + main) using the variant.
pub fn demo_shell(v: Variant, buttons: HeroButtons) -> Element {
    rsx! {
        div { class: "{v.prefix}",
            {nav_bar(&v)}
            main { class: "{v.cls(\"main\")}",
                {hero(&v, &buttons)}
                {game_grid(&v)}
                p { class: "{v.cls(\"label\")}", "{v.label}" }
            }
        }
    }
}

/// Top navigation bar with logo, page links, and actions.
fn nav_bar(v: &Variant) -> Element {
    rsx! {
        nav { class: "{v.cls(\"nav\")}",
            span { class: "{v.cls(\"logo\")}", "PYCKX" }
            {nav_links(v)}
            {nav_actions(v)}
        }
    }
}

/// Navigation links (Home / Study / Collection).
fn nav_links(v: &Variant) -> Element {
    let pill = v.cls("pill");
    rsx! {
        div { class: "{v.cls(\"nav-links\")}",
            span { class: "{pill} active", "Home" }
            span { class: "{pill}", "Study" }
            span { class: "{pill}", "Collection" }
        }
    }
}

/// Right-side nav action buttons.
fn nav_actions(v: &Variant) -> Element {
    rsx! {
        div { class: "{v.cls(\"nav-actions\")}",
            button { class: "{v.cls(\"ghost\")}", "D" }
            button { class: "{v.cls(\"btn\")}", "Logout" }
        }
    }
}

/// Hero header with title, subtitle, and action buttons.
fn hero(v: &Variant, buttons: &HeroButtons) -> Element {
    rsx! {
        header { class: "{v.cls(\"hero\")}",
            h1 { class: "{v.cls(\"hero-title\")}", "Study" }
            p { class: "{v.cls(\"hero-sub\")}",
                "Choose a learning game"
            }
            div { class: "{v.cls(\"hero-actions\")}",
                button { class: "{v.hero_primary_class}",
                    "{buttons.primary}"
                }
                button { class: "{v.hero_secondary_class}",
                    "{buttons.secondary}"
                }
            }
        }
    }
}

/// Grid of game cards using the shared `GAMES` data.
fn game_grid(v: &Variant) -> Element {
    rsx! {
        div { class: "{v.cls(\"grid\")}",
            for g in GAMES.iter() {
                {game_card(v, g)}
            }
        }
    }
}

/// Single game card with icon, title, description, footer.
fn game_card(v: &Variant, g: &Game) -> Element {
    rsx! {
        div { class: "{v.cls(\"card\")}",
            div { class: "{v.cls(\"card-icon\")}", "O" }
            h3 { class: "{v.cls(\"card-title\")}", "{g.name}" }
            p { class: "{v.cls(\"card-desc\")}", "{g.desc}" }
            div { class: "{v.cls(\"card-footer\")}",
                span { class: "{v.cls(\"badge\")}",
                    "{g.count} sets"
                }
                button { class: "{v.cls(\"btn-sm\")}", "Play" }
            }
        }
    }
}
