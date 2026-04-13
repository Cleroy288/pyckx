//! Card that represents a single game set in the list.

use crate::ui::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant,
    ConfirmDialog, ItemCard,
};
use dioxus::prelude::*;

/// Card for any game set (QCM, flashcard, etc.).
#[component]
pub fn GameSetCard(
    /// Set display name.
    name: String,
    /// Difficulty level string.
    level: String,
    /// Subject tags.
    subjects: Vec<String>,
    /// Number of items in the set.
    count: usize,
    /// Singular/plural label for items.
    item_label: String,
    /// Called when the "Play" button is clicked.
    on_play: EventHandler<()>,
    /// Called when deletion is confirmed.
    on_delete: EventHandler<()>,
) -> Element {
    let mut confirm_open = use_signal(|| false);
    let set_name = name.clone();
    let level_variant = level_badge(&level);

    rsx! {
        ItemCard {
            name: name,
            header_end: rsx! {
                Badge { text: level, variant: level_variant }
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
                    on_click: move |_| confirm_open.set(true),
                }
            },
            span {
                class: "text-[0.8125rem] \
                    text-[var(--color-text-secondary)]",
                "{count} {item_label}"
            }
            if !subjects.is_empty() {
                div { class: "flex flex-wrap gap-1.5",
                    for (index, subject) in subjects.iter().enumerate() {
                        Badge { key: "{index}", text: subject.clone() }
                    }
                }
            }
        }
        ConfirmDialog {
            open: confirm_open,
            title: "Delete",
            message: "Delete \"{set_name}\"? This cannot be undone.",
            confirm_label: "Delete",
            on_confirm: move |_| on_delete.call(()),
        }
    }
}

/// Map a level string to its badge variant.
fn level_badge(level: &str) -> BadgeVariant {
    match level {
        "easy" => BadgeVariant::Success,
        "hard" => BadgeVariant::Error,
        _ => BadgeVariant::Warning,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_badge_easy_is_success() {
        assert!(matches!(
            level_badge("easy"),
            BadgeVariant::Success
        ));
    }

    #[test]
    fn test_level_badge_hard_is_error() {
        assert!(matches!(
            level_badge("hard"),
            BadgeVariant::Error
        ));
    }

    #[test]
    fn test_level_badge_medium_is_warning() {
        assert!(matches!(
            level_badge("medium"),
            BadgeVariant::Warning
        ));
    }

    #[test]
    fn test_level_badge_unknown_is_warning() {
        assert!(matches!(
            level_badge("???"),
            BadgeVariant::Warning
        ));
    }
}
