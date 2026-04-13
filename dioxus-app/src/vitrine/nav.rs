//! `VitrineNav` — sticky top navigation.

use dioxus::prelude::*;

use super::buttons::{CtaButton, GhostButton, GhostTarget};
use crate::home::NavLogo;

/// Right arrow glyph used in the CTA highlight.
const ARROW_GLYPH: &str = " \u{2192}";

/// Sticky nav bar: logo, anchor links, auth buttons.
#[component]
pub fn VitrineNav() -> Element {
    rsx! {
        nav {
            Link { to: "/", class: "logo-wrap no-underline",
                NavLogo {}
            }
            NavLinks {}
            NavActions {}
        }
    }
}

/// In-page anchor links (Jeux, Tarifs).
#[component]
fn NavLinks() -> Element {
    rsx! {
        div { class: "nav-links",
            a { href: "#jeux", "Jeux" }
            a { href: "#pricing", "Tarifs" }
        }
    }
}

/// Login (ghost) + start (CTA) buttons.
#[component]
fn NavActions() -> Element {
    rsx! {
        div { class: "nav-actions",
            GhostButton {
                label: "Login",
                target: GhostTarget::Route("/login"),
            }
            CtaButton {
                label: "Commencer",
                highlight: ARROW_GLYPH,
                to: "/login",
            }
        }
    }
}
