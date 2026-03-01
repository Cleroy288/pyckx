// ** game_results.rs **
// ==> Score display + per-question breakdown

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{
    btn_click, Button, ButtonVariant,
};
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/shared/game_results.module.css"
);

/// Single result item
#[derive(Clone)]
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
    #[prop(into)]
    items: Vec<ResultItem>,
    /// Called when replay is clicked
    #[prop(into)]
    on_replay: Callback<()>,
    /// Called when back is clicked
    #[prop(into)]
    on_back: Callback<()>,
) -> impl IntoView {
    let pct = if total > 0 {
        (score as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    view! {
        <div class=style::results>
            <div class=style::score_section>
                <h2 class=style::score>
                    {score} "/" {total}
                </h2>
                <span class=style::percent>
                    {pct}"% correct"
                </span>
            </div>
            <div class=style::items>
                <For
                    each=move || items.clone()
                    key=|r| r.question.clone()
                    let:item
                >
                    <div class=style::item>
                        <div class=style::item_header>
                            <Badge
                                text=if item.correct {
                                    "Correct".to_string()
                                } else {
                                    "Wrong".to_string()
                                }
                                variant=if item.correct {
                                    BadgeVariant::Success
                                } else {
                                    BadgeVariant::Error
                                }
                            />
                            <span class=style::question>
                                {item.question.clone()}
                            </span>
                        </div>
                        <p class=style::explanation>
                            {item.explanation.clone()}
                        </p>
                    </div>
                </For>
            </div>
            <div class=style::actions>
                <Button
                    text="Play Again"
                    on_click=btn_click(move |_| on_replay.run(()))
                />
                <Button
                    text="Back to List"
                    variant=ButtonVariant::Outline
                    on_click=btn_click(move |_| on_back.run(()))
                />
            </div>
        </div>
    }
}
