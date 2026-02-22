// ** game_progress.rs **
// ==> Question X/Y progress bar during gameplay

use crate::components::ui::progress::Progress;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/shared/game_progress.module.css"
);

/// Game progress indicator
#[component]
pub fn GameProgress(
    /// Current question index (0-based)
    #[prop(into)]
    current: Signal<usize>,
    /// Total number of questions
    total: usize,
) -> impl IntoView {
    let label = Signal::derive(move || {
        format!(
            "Question {} / {}",
            current.get() + 1,
            total
        )
    });

    let pct = Signal::derive(move || {
        if total == 0 {
            return 0.0;
        }
        ((current.get() + 1) as f64 / total as f64)
            * 100.0
    });

    view! {
        <div class=style::wrapper>
            <span class=style::label>
                {move || label.get()}
            </span>
            <Progress value=pct />
        </div>
    }
}
