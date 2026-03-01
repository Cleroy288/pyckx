// ** game_progress.rs **
// ==> Question X/Y progress bar during gameplay

use crate::components::ui::progress::Progress;
use dioxus::prelude::*;

/// Game progress indicator
#[component]
pub fn GameProgress(
    /// Current question index (0-based)
    current: Signal<usize>,
    /// Total number of questions
    total: usize,
) -> Element {
    let label = use_memo(move || {
        format!(
            "Question {} / {}",
            (current)() + 1,
            total
        )
    });

    let pct = use_memo(move || {
        if total == 0 {
            return 0.0;
        }
        (((current)() + 1) as f64 / total as f64)
            * 100.0
    });

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
