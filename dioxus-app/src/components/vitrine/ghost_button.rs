//! GhostButton — outlined ghost-style button

use dioxus::prelude::*;

/// Target for a ghost button: route or anchor
#[derive(Clone, PartialEq)]
pub enum GhostTarget {
    /// Internal route via `Link`
    Route(&'static str),
    /// Anchor scroll via `<a href>`
    Anchor(&'static str),
}

/// Outlined ghost button for secondary actions
#[component]
pub fn GhostButton(
    label: &'static str,
    target: GhostTarget,
) -> Element {
    match target {
        GhostTarget::Route(to) => rsx! {
            Link { to: to,
                span { class: "btn-ghost",
                    "{label}"
                }
            }
        },
        GhostTarget::Anchor(href) => rsx! {
            a { href: href,
                class: "btn-ghost",
                "{label}"
            }
        },
    }
}
