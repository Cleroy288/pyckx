//! Floating decorative stickers behind the hero.

use dioxus::prelude::*;

/// Renders decorative floating shapes over the background.
#[component]
pub fn DecoStickers() -> Element {
    rsx! {
        div { class: "home-deco",
            div { class: "home-sticker home-sticker-a" }
            div { class: "home-sticker home-sticker-b" }
            div { class: "home-sticker home-sticker-c" }
        }
    }
}
