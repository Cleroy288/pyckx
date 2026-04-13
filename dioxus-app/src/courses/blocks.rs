//! Leaf renderer for `ContentBlock` variants.
//!
//! `render_block` is a pure function from a block payload
//! to a Dioxus `Element`. No state, no side effects.

use dioxus::prelude::*;

use super::types::ContentBlock;

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

/// CSS class for h2 titles inside a module
const TITLE_CLS: &str = "text-[1.1rem] font-semibold \
    text-[var(--color-text-primary)] mt-2 mb-0";

/// CSS class for h3 subtitles
const SUBTITLE_CLS: &str = "text-base font-medium \
    text-[var(--color-text-secondary)] mt-1 mb-0";

/// CSS class for paragraph text
const TEXT_CLS: &str = "text-[0.9rem] \
    text-[var(--color-text-secondary)] \
    leading-relaxed m-0";

/// CSS class for code blocks
const SCHEMA_CLS: &str = "bg-black/30 \
    border border-[var(--color-border)] \
    p-4 overflow-x-auto text-[0.85rem] \
    text-[var(--color-text-primary)] m-0";

/// Label a content block for diagnostics and keys.
pub fn block_label(
    block: &ContentBlock,
) -> &'static str {
    match block {
        ContentBlock::Title { .. } => "title",
        ContentBlock::Subtitle { .. } => "subtitle",
        ContentBlock::Text { .. } => "text",
        ContentBlock::Schema { .. } => "schema",
        ContentBlock::QcmSet { .. } => "qcm",
        ContentBlock::TrueFalseSet { .. } => "true_false",
        ContentBlock::FlashcardSet { .. } => "flashcards",
    }
}

/// Render any content block variant to an `Element`.
pub fn render_block(block: &ContentBlock) -> Element {
    match block {
        ContentBlock::Title { content } => rsx! {
            h2 { class: TITLE_CLS, "{content}" }
        },
        ContentBlock::Subtitle { content } => rsx! {
            h3 { class: SUBTITLE_CLS, "{content}" }
        },
        ContentBlock::Text { content } => rsx! {
            p { class: TEXT_CLS, "{content}" }
        },
        ContentBlock::Schema {
            language,
            content,
        } => rsx! {
            pre { class: SCHEMA_CLS,
                code { "// {language}\n{content}" }
            }
        },
        ContentBlock::QcmSet { data } => {
            render_game_preview(
                "QCM",
                &data.name,
                &data.description,
            )
        }
        ContentBlock::TrueFalseSet { data } => {
            render_game_preview(
                "True/False",
                &data.name,
                &data.description,
            )
        }
        ContentBlock::FlashcardSet { data } => {
            render_game_preview(
                "Flashcards",
                &data.name,
                &data.description,
            )
        }
    }
}

/// Render a compact preview card for a game set block.
fn render_game_preview(
    label: &str,
    name: &str,
    description: &str,
) -> Element {
    rsx! {
        div { class: GAME_CARD,
            h4 { class: GAME_TITLE,
                "{label}: {name}"
            }
            p { class: GAME_DESC, "{description}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_label_title() {
        let b = ContentBlock::Title {
            content: "x".into(),
        };
        assert_eq!(block_label(&b), "title");
    }

    #[test]
    fn test_block_label_subtitle() {
        let b = ContentBlock::Subtitle {
            content: "x".into(),
        };
        assert_eq!(block_label(&b), "subtitle");
    }

    #[test]
    fn test_block_label_text() {
        let b = ContentBlock::Text {
            content: "x".into(),
        };
        assert_eq!(block_label(&b), "text");
    }

    #[test]
    fn test_block_label_schema() {
        let b = ContentBlock::Schema {
            language: "rs".into(),
            content: "".into(),
        };
        assert_eq!(block_label(&b), "schema");
    }

    #[test]
    fn test_block_label_qcm_set() {
        let b = ContentBlock::QcmSet {
            data: crate::courses::types::QcmSetPayload {
                name: "n".into(),
                description: "d".into(),
                level: "l".into(),
                subjects: vec![],
                questions: vec![],
            },
        };
        assert_eq!(block_label(&b), "qcm");
    }
}
