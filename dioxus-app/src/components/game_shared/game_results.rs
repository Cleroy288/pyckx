//! Score display + per-question breakdown

use crate::ui::{Badge, BadgeVariant};
use crate::ui::{
    Button, ButtonVariant,
};
use dioxus::prelude::*;

/// Single result item
#[derive(Clone, PartialEq)]
pub struct ResultItem {
    /// Question text
    pub question: String,
    /// Whether the answer was correct
    pub correct: bool,
    /// Explanation
    pub explanation: String,
}

/// Game results display
#[component]
pub fn GameResults(
    /// Score (correct answers)
    score: usize,
    /// Total questions
    total: usize,
    /// Per-question results
    items: Vec<ResultItem>,
    /// Called when replay is clicked
    on_replay: EventHandler<()>,
    /// Called when back is clicked
    on_back: EventHandler<()>,
) -> Element {
    let pct = if total > 0 {
        (score as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    rsx! {
        div {
            class: "flex flex-col gap-6 \
                max-w-[700px] mx-auto",
            // Score section
            div {
                class: "text-center p-6 \
                    bg-[var(--card)] \
                    border-2 border-[var(--color-border)] \
                    rounded-[20px]",
                h2 {
                    class: "m-0 text-[2.5rem] \
                        font-bold \
                        text-[var(--color-text-primary)]",
                    "{score}/{total}"
                }
                span {
                    class: "text-base \
                        text-[var(--color-text-secondary)]",
                    "{pct}% correct"
                }
            }
            // Result items
            div { class: "flex flex-col gap-3",
                for (index, item) in items.iter().enumerate() {
                    div { key: "{index}",
                        {render_item(item)}
                    }
                }
            }
            // Actions
            div {
                class: "flex gap-3 justify-center",
                Button {
                    text: "Play Again",
                    on_click: move |_| on_replay.call(()),
                }
                Button {
                    text: "Back to List",
                    variant: ButtonVariant::Outline,
                    on_click: move |_| on_back.call(()),
                }
            }
        }
    }
}

/// Render a single result item row
fn render_item(item: &ResultItem) -> Element {
    let variant = if item.correct {
        BadgeVariant::Success
    } else {
        BadgeVariant::Error
    };
    let label = if item.correct {
        "Correct"
    } else {
        "Wrong"
    };

    rsx! {
        div {
            class: "p-4 bg-[var(--card)] \
                border-2 border-[var(--color-border)] \
                rounded-[20px]",
            div {
                class: "flex items-center gap-3 mb-2",
                Badge {
                    text: label,
                    variant: variant,
                }
                span {
                    class: "text-sm font-medium \
                        text-[var(--color-text-primary)]",
                    "{item.question}"
                }
            }
            p {
                class: "m-0 text-[0.8125rem] \
                    text-[var(--color-text-secondary)]",
                "{item.explanation}"
            }
        }
    }
}
