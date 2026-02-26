//! DvdCard — single DVD display with edit/delete

use crate::components::ui::badge::Badge;
use crate::components::ui::button::{
    Button, ButtonSize, ButtonVariant,
};
use crate::components::ui::card::ItemCard;
use crate::domain::collection_types::Dvd;
use dioxus::prelude::*;

/// Display a single DVD with actions
#[component]
pub fn DvdCard(
    /// DVD data to display
    dvd: Dvd,
    /// Called when edit is clicked
    on_edit: EventHandler<String>,
    /// Called when delete is clicked
    on_delete: EventHandler<String>,
) -> Element {
    let edit_id = dvd.id.clone();
    let del_id = dvd.id.clone();
    let actors = dvd.actors.clone();
    let actors_str = dvd.actors.join(", ");

    rsx! {
        ItemCard {
            name: dvd.name.clone(),
            header_end: rsx! {
                Badge {
                    text: dvd.year.to_string(),
                }
            },
            actions: rsx! {
                Button {
                    text: "Edit".to_string(),
                    size: ButtonSize::Small,
                    on_click: {
                        let id = edit_id.clone();
                        EventHandler::new(move |_| {
                            on_edit.call(id.clone())
                        })
                    },
                }
                Button {
                    text: "Delete".to_string(),
                    size: ButtonSize::Small,
                    variant: ButtonVariant::Outline,
                    on_click: {
                        let id = del_id.clone();
                        EventHandler::new(move |_| {
                            on_delete.call(id.clone())
                        })
                    },
                }
            },
            // Director
            if let Some(ref dir) = dvd.realisator {
                p {
                    class: "m-0 text-sm \
                        text-[var(--color-text-secondary)] \
                        overflow-hidden \
                        text-ellipsis whitespace-nowrap",
                    "{dir}"
                }
            }
            // Actors
            if !actors.is_empty() {
                p {
                    class: "m-0 text-sm italic \
                        text-[var(--color-text-secondary)] \
                        overflow-hidden \
                        text-ellipsis whitespace-nowrap",
                    "{actors_str}"
                }
            }
            // Genre badge
            if let Some(ref genre) = dvd.genre {
                Badge {
                    text: genre.clone(),
                }
            }
        }
    }
}
