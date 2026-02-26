// ** game_set_card.rs **
// ==> Generic card for any game set (QCM, flashcard...)

use crate::components::ui::badge::{
    Badge, BadgeVariant,
};
use crate::components::ui::button::{
    Button, ButtonSize, ButtonVariant,
};
use crate::components::ui::card::ItemCard;
use dioxus::prelude::*;

/// Generic game set card
#[component]
pub fn GameSetCard(
    /// Set name
    name: String,
    /// Difficulty level
    level: String,
    /// Subject badges
    subjects: Vec<String>,
    /// Number of items (questions/cards)
    count: usize,
    /// Item label ("questions", "cards", etc.)
    item_label: String,
    /// Called when play button clicked
    on_play: EventHandler<()>,
    /// Called when delete button clicked
    on_delete: EventHandler<()>,
) -> Element {
    let lvl_variant = match level.as_str() {
        "easy" => BadgeVariant::Success,
        "hard" => BadgeVariant::Error,
        _ => BadgeVariant::Warning,
    };

    rsx! {
        ItemCard {
            name: name,
            header_end: rsx! {
                Badge {
                    text: level,
                    variant: lvl_variant,
                }
            },
            actions: rsx! {
                Button {
                    text: "Play",
                    size: ButtonSize::Small,
                    on_click: move |_| on_play.call(()),
                }
                Button {
                    text: "Delete",
                    size: ButtonSize::Small,
                    variant: ButtonVariant::Outline,
                    on_click: move |_| {
                        on_delete.call(())
                    },
                }
            },
            span {
                class: "text-[0.8125rem] \
                    text-[var(--color-text-secondary)]",
                "{count} {item_label}"
            }
            if !subjects.is_empty() {
                div {
                    class: "flex flex-wrap gap-1.5",
                    for s in subjects.iter() {
                        Badge { text: s.clone() }
                    }
                }
            }
        }
    }
}
