//! End-of-run results screen.

use crate::ui::{Badge, BadgeVariant, Button, ButtonVariant};
use dioxus::prelude::*;

const PERCENT: f64 = 100.0;

/// Summary of a single question's outcome.
#[derive(Clone, PartialEq)]
pub struct ResultItem {
    /// Question text.
    pub question: String,
    /// Whether the player answered correctly.
    pub correct: bool,
    /// Explanation shown with the result.
    pub explanation: String,
}

/// Final screen: score + per-question breakdown.
#[component]
pub fn GameResults(
    /// Correct answer count.
    score: usize,
    /// Total question count.
    total: usize,
    /// Detailed results for each question.
    items: Vec<ResultItem>,
    /// Called when the player wants to replay.
    on_replay: EventHandler<()>,
    /// Called when the player returns to the list.
    on_back: EventHandler<()>,
) -> Element {
    let pct = compute_score_pct(score, total);

    rsx! {
        div {
            class: "flex flex-col gap-6 \
                max-w-[700px] mx-auto",
            {score_card(score, total, pct)}
            div { class: "flex flex-col gap-3",
                for (index, item) in items.iter().enumerate() {
                    div { key: "{index}", {render_item(item)} }
                }
            }
            div { class: "flex gap-3 justify-center",
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

/// Render the top score card.
fn score_card(score: usize, total: usize, pct: u32) -> Element {
    rsx! {
        div {
            class: "text-center p-6 bg-[var(--card)] \
                border-2 border-[var(--color-border)] \
                rounded-[20px]",
            h2 {
                class: "m-0 text-[2.5rem] font-bold \
                    text-[var(--color-text-primary)]",
                "{score}/{total}"
            }
            span {
                class: "text-base \
                    text-[var(--color-text-secondary)]",
                "{pct}% correct"
            }
        }
    }
}

/// Render one row of the results breakdown.
fn render_item(item: &ResultItem) -> Element {
    let (variant, label) = row_badge(item.correct);
    rsx! {
        div {
            class: "p-4 bg-[var(--card)] \
                border-2 border-[var(--color-border)] \
                rounded-[20px]",
            div { class: "flex items-center gap-3 mb-2",
                Badge { text: label, variant: variant }
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

/// Pick the badge variant + label for a result row.
fn row_badge(correct: bool) -> (BadgeVariant, &'static str) {
    if correct {
        (BadgeVariant::Success, "Correct")
    } else {
        (BadgeVariant::Error, "Wrong")
    }
}

/// Compute the integer score percentage.
fn compute_score_pct(score: usize, total: usize) -> u32 {
    if total == 0 {
        return 0;
    }
    (score as f64 / total as f64 * PERCENT) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_score_pct_all_correct_returns_100() {
        assert_eq!(compute_score_pct(10, 10), 100);
    }

    #[test]
    fn test_compute_score_pct_half_returns_50() {
        assert_eq!(compute_score_pct(5, 10), 50);
    }

    #[test]
    fn test_compute_score_pct_zero_total_returns_zero() {
        assert_eq!(compute_score_pct(0, 0), 0);
    }

    #[test]
    fn test_row_badge_correct_returns_success_label() {
        let (_, label) = row_badge(true);
        assert_eq!(label, "Correct");
    }

    #[test]
    fn test_row_badge_correct_returns_success_variant() {
        let (variant, _) = row_badge(true);
        assert!(matches!(variant, BadgeVariant::Success));
    }

    #[test]
    fn test_row_badge_wrong_returns_error_label() {
        let (_, label) = row_badge(false);
        assert_eq!(label, "Wrong");
    }

    #[test]
    fn test_row_badge_wrong_returns_error_variant() {
        let (variant, _) = row_badge(false);
        assert!(matches!(variant, BadgeVariant::Error));
    }
}
