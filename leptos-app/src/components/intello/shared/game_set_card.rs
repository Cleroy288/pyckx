// ** game_set_card.rs **
// ==> Generic card for any game set (QCM, flashcard…)

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{
    btn_click, Button, ButtonSize, ButtonVariant,
};
use crate::components::ui::card::ItemCard;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/shared/\
     game_set_card.module.css"
);

/// Generic game set card
#[component]
pub fn GameSetCard(
    /// Set name
    #[prop(into)]
    name: String,
    /// Difficulty level
    #[prop(into)]
    level: String,
    /// Subject badges
    #[prop(into)]
    subjects: Vec<String>,
    /// Number of items (questions/cards)
    count: usize,
    /// Item label ("questions", "cards", etc.)
    #[prop(into)]
    item_label: String,
    /// Called when play button clicked
    #[prop(into)]
    on_play: Callback<()>,
    /// Called when delete button clicked
    #[prop(into)]
    on_delete: Callback<()>,
) -> impl IntoView {
    let lvl_variant = match level.as_str() {
        "easy" => BadgeVariant::Success,
        "hard" => BadgeVariant::Error,
        _ => BadgeVariant::Warning,
    };

    view! {
        <ItemCard
            name=name
            header_end=view! {
                <Badge
                    text=level
                    variant=lvl_variant
                />
            }.into_any()
            actions=view! {
                <Button
                    text="Play"
                    size=ButtonSize::Small
                    on_click=btn_click(move |_| {
                        on_play.run(())
                    })
                />
                <Button
                    text="Delete"
                    size=ButtonSize::Small
                    variant=ButtonVariant::Outline
                    on_click=btn_click(move |_| {
                        on_delete.run(())
                    })
                />
            }.into_any()
        >
            <span class=style::meta>
                {count} " " {item_label}
            </span>
            {render_subjects(subjects)}
        </ItemCard>
    }
}

/// Renders subject badges if non-empty
fn render_subjects(
    subjects: Vec<String>,
) -> impl IntoView {
    let subs = subjects.clone();
    view! {
        <Show when=move || !subs.is_empty()>
            <div class=style::subjects>
                {subjects.iter().map(|s| view! {
                    <Badge text=s.clone() />
                }).collect_view()}
            </div>
        </Show>
    }
}
