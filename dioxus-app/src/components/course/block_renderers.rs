//! Leaf renderers for individual content block types

use dioxus::prelude::*;

/// CSS class for game set preview cards
const GAME_CARD: &str =
    "p-4 border border-dashed \
     border-[color-mix(in_srgb,var(--primary)_30%,transparent)] \
     bg-[color-mix(in_srgb,var(--primary)_5%,transparent)]";

/// CSS class for game set title
const GAME_TITLE: &str =
    "text-[0.95rem] \
     text-[var(--color-text-secondary)] \
     m-0 mb-1";

/// CSS class for game set description
const GAME_DESC: &str =
    "text-[0.85rem] \
     text-[var(--color-text-secondary)] m-0";

/// Render a title block
pub fn render_title(content: &str) -> Element {
    rsx! {
        h2 {
            class: "text-[1.1rem] font-semibold \
                text-[var(--color-text-primary)] \
                mt-2 mb-0",
            "{content}"
        }
    }
}

/// Render a subtitle block
pub fn render_subtitle(
    content: &str,
) -> Element {
    rsx! {
        h3 {
            class: "text-base font-medium \
                text-[var(--color-text-secondary)] \
                mt-1 mb-0",
            "{content}"
        }
    }
}

/// Render a text paragraph block
pub fn render_text(content: &str) -> Element {
    rsx! {
        p {
            class: "text-[0.9rem] \
                text-[var(--color-text-secondary)] \
                leading-relaxed m-0",
            "{content}"
        }
    }
}

/// Render a code/schema block
pub fn render_schema(
    language: &str,
    content: &str,
) -> Element {
    rsx! {
        pre {
            class: "bg-black/30 \
                border border-[var(--color-border)] \
                p-4 overflow-x-auto \
                text-[0.85rem] \
                text-[var(--color-text-primary)] \
                m-0",
            code {
                "// {language}\n{content}"
            }
        }
    }
}

/// Render a game set preview card
pub fn render_game_card(
    label: &str,
    name: &str,
    description: &str,
) -> Element {
    rsx! {
        div { class: GAME_CARD,
            h4 { class: GAME_TITLE,
                "{label}: {name}"
            }
            p { class: GAME_DESC,
                "{description}"
            }
        }
    }
}
