//! VitrineNav — sticky navigation bar

use crate::components::logo::NavLogo;
use crate::components::vitrine::{
    CtaButton, GhostButton, GhostTarget,
};
use dioxus::prelude::*;

/// Sticky nav bar with logo, links, buttons
#[component]
pub fn VitrineNav() -> Element {
    rsx! {
        nav {
            Link { to: "/",
                class: "logo-wrap no-underline",
                NavLogo {}
            }
            div { class: "nav-links",
                a { href: "#jeux", "Jeux" }
                a { href: "#pricing", "Tarifs" }
            }
            div {
                class: "nav-actions",
                GhostButton {
                    label: "Login",
                    target: GhostTarget::Route(
                        "/login"
                    ),
                }
                CtaButton {
                    label: "Commencer",
                    highlight: " \u{2192}",
                    to: "/login",
                }
            }
        }
    }
}
