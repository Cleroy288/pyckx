//! DVD card — renders one DVD with edit/delete actions.

use crate::collection::types::Dvd;
use crate::ui::{
    Badge, Button, ButtonSize, ButtonVariant, ItemCard,
};
use dioxus::prelude::*;

/// Tailwind classes for inline secondary text.
const SECONDARY_TEXT: &str = "m-0 text-sm \
    text-[var(--color-text-secondary)] \
    overflow-hidden text-ellipsis whitespace-nowrap";

/// Tailwind classes for italic secondary text.
const SECONDARY_ITALIC: &str = "m-0 text-sm italic \
    text-[var(--color-text-secondary)] \
    overflow-hidden text-ellipsis whitespace-nowrap";

/// Render a single DVD with edit/delete buttons.
#[component]
pub fn DvdCard(
    /// DVD data to display.
    dvd: Dvd,
    /// Fired with the DVD id when Edit is clicked.
    on_edit: EventHandler<String>,
    /// Fired with the DVD id when Delete is clicked.
    on_delete: EventHandler<String>,
) -> Element {
    let actors_str = dvd.actors.join(", ");
    let has_actors = !dvd.actors.is_empty();

    rsx! {
        ItemCard {
            name: dvd.name.clone(),
            header_end: rsx! {
                Badge { text: dvd.year.to_string() }
            },
            actions: action_buttons(
                dvd.id.clone(),
                on_edit,
                on_delete,
            ),
            if let Some(ref dir) = dvd.realisator {
                p { class: SECONDARY_TEXT, "{dir}" }
            }
            if has_actors {
                p {
                    class: SECONDARY_ITALIC,
                    "{actors_str}"
                }
            }
            if let Some(ref genre) = dvd.genre {
                Badge { text: genre.clone() }
            }
        }
    }
}

/// Build the edit and delete buttons for a card.
fn action_buttons(
    id: String,
    on_edit: EventHandler<String>,
    on_delete: EventHandler<String>,
) -> Element {
    let edit_id = id.clone();
    let del_id = id;
    rsx! {
        Button {
            text: "Edit".to_string(),
            size: ButtonSize::Small,
            on_click: EventHandler::new(move |_| {
                on_edit.call(edit_id.clone())
            }),
        }
        Button {
            text: "Delete".to_string(),
            size: ButtonSize::Small,
            variant: ButtonVariant::Outline,
            on_click: EventHandler::new(move |_| {
                on_delete.call(del_id.clone())
            }),
        }
    }
}
