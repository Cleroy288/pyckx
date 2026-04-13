//! CourseCard — uses ItemCard shell like game set cards

use crate::components::ui::badge::Badge;
use crate::components::ui::button::{
    Button, ButtonSize, ButtonVariant,
};
use crate::components::ui::card::ItemCard;
use dioxus::prelude::*;

/// Course card with name, description, actions
#[component]
pub fn CourseCard(
    /// Course name
    name: String,
    /// Course description
    description: String,
    /// Creation date
    created_at: String,
    /// Called when view button clicked
    on_view: EventHandler<()>,
    /// Called when delete button clicked
    on_delete: EventHandler<()>,
) -> Element {
    rsx! {
        ItemCard {
            name: name,
            header_end: rsx! {
                Badge { text: "Course" }
            },
            actions: rsx! {
                Button {
                    text: "View",
                    size: ButtonSize::Small,
                    on_click: move |_| {
                        on_view.call(())
                    },
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
            p {
                class: "text-sm \
                    text-[var(--color-text-secondary)] \
                    m-0 leading-snug",
                "{description}"
            }
            span {
                class: "text-xs \
                    text-[var(--color-text-secondary)]",
                "{created_at}"
            }
        }
    }
}
