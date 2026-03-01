// ** course_card.rs **
// ==> Displays a single course with metadata

use crate::components::ui::button::{
    Button, ButtonSize, ButtonVariant,
};
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
        div {
            class: "flex flex-col gap-4 p-5 \
                border border-[var(--color-border)] \
                bg-[var(--glass-bg)] \
                backdrop-blur-[20px]",
            div { class: "flex flex-col gap-2",
                h3 {
                    class: "text-[1.1rem] \
                        font-semibold \
                        text-[var(--color-text-primary)] \
                        m-0",
                    "{name}"
                }
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
            div { class: "flex gap-2",
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
            }
        }
    }
}
