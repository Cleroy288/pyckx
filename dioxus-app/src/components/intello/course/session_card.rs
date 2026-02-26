// ** session_card.rs **
// ==> Displays a study session with status and actions

use crate::components::ui::badge::{
    Badge, BadgeVariant,
};
use crate::components::ui::button::{
    Button, ButtonSize,
};
use dioxus::prelude::*;

/// Study session card
#[component]
pub fn SessionCard(
    /// Session topic
    topic: String,
    /// Session status (pending, completed, etc.)
    status: String,
    /// Creation date
    created_at: String,
    /// Called when view button clicked
    on_view: EventHandler<()>,
) -> Element {
    let variant = match status.as_str() {
        "completed" => BadgeVariant::Success,
        "failed" => BadgeVariant::Error,
        _ => BadgeVariant::Warning,
    };

    rsx! {
        div {
            class: "flex flex-col gap-3 p-4 \
                border border-[var(--color-border)] \
                bg-[var(--glass-bg)] \
                backdrop-blur-[20px]",
            div {
                class: "flex justify-between \
                    items-center gap-2",
                h4 {
                    class: "text-base font-semibold \
                        text-[var(--color-text-primary)] \
                        m-0",
                    "{topic}"
                }
                Badge {
                    text: status,
                    variant: variant,
                }
            }
            span {
                class: "text-xs \
                    text-[var(--color-text-secondary)]",
                "{created_at}"
            }
            div { class: "flex gap-2",
                Button {
                    text: "View",
                    size: ButtonSize::Small,
                    on_click: move |_| {
                        on_view.call(())
                    },
                }
            }
        }
    }
}
