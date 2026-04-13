//! GameCard — Brutalist game card with hover effects

use dioxus::prelude::*;

/// CSS class for a game card
fn card_class(featured: bool) -> &'static str {
    if featured {
        "gcard gcard-feat"
    } else {
        "gcard"
    }
}

/// Brutalist game card with ink shadow and accent
/// line on hover. Renders as `<button>` when
/// `on_click` is provided, `<div>` otherwise.
#[component]
pub fn GameCard(
    /// Short tag label (e.g. "01")
    tag: String,
    /// Game name
    name: String,
    /// Short description
    desc: String,
    /// Featured style (accent background)
    #[props(default = false)]
    featured: bool,
    /// Optional click handler
    #[props(optional)]
    on_click: Option<EventHandler<()>>,
) -> Element {
    let cls = card_class(featured);
    let inner = if featured {
        rsx! {
            div { class: "gcard-feat-content",
                span { class: "gcard-tag", "{tag}" }
                h3 { "{name}" }
                p { "{desc}" }
                span { class: "gcard-arrow", "\u{2192}" }
            }
        }
    } else {
        rsx! {
            span { class: "gcard-tag", "{tag}" }
            h3 { "{name}" }
            p { "{desc}" }
            span { class: "gcard-arrow", "\u{2192}" }
        }
    };

    match on_click {
        Some(handler) => rsx! {
            button {
                class: "{cls}",
                onclick: move |_| handler.call(()),
                {inner}
            }
        },
        None => rsx! {
            div { class: "{cls}", {inner} }
        },
    }
}
