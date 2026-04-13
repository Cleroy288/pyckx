//! Progress bar shown during gameplay.

use crate::ui::Progress;
use dioxus::prelude::*;

const PERCENT: f64 = 100.0;

/// Question counter with a progress bar.
#[component]
pub fn GameProgress(
    /// Current index (0-based).
    current: Signal<usize>,
    /// Total number of items.
    total: usize,
) -> Element {
    let label = use_memo(move || {
        format!("Question {} / {}", current() + 1, total)
    });
    let pct = use_memo(move || compute_pct(current(), total));

    rsx! {
        div { class: "flex flex-col gap-2",
            span {
                class: "text-sm font-semibold \
                    text-[var(--color-text-primary)]",
                "{label}"
            }
            Progress { value: pct }
        }
    }
}

/// Compute completion percentage for `current` of `total`.
fn compute_pct(current: usize, total: usize) -> f64 {
    if total == 0 {
        return 0.0;
    }
    ((current + 1) as f64 / total as f64) * PERCENT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pct_with_zero_total_returns_zero() {
        assert_eq!(compute_pct(0, 0), 0.0);
    }

    #[test]
    fn test_compute_pct_first_question_of_ten() {
        assert_eq!(compute_pct(0, 10), 10.0);
    }

    #[test]
    fn test_compute_pct_last_question_is_full() {
        assert_eq!(compute_pct(9, 10), 100.0);
    }

    #[test]
    fn test_compute_pct_midway_through_run() {
        assert_eq!(compute_pct(4, 10), 50.0);
    }
}
