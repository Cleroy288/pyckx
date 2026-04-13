//! SessionCard — brutalist card for study sessions

use crate::components::ui::badge::{
    Badge, BadgeVariant,
};
use crate::components::ui::button::{
    Button, ButtonSize,
};
use dioxus::prelude::*;

/// Card shell — same style as ItemCard
const CARD_CLS: &str = "\
    flex flex-col gap-4 p-8 \
    bg-[var(--card)] \
    border-2 border-[var(--border)] \
    rounded-[20px] overflow-hidden \
    relative cursor-pointer \
    transition-[transform,border-color,box-shadow] \
    duration-300 \
    hover:-translate-y-2 \
    hover:border-[var(--color-primary)] \
    hover:shadow-[0_20px_50px_var(--glow)] \
    after:content-[''] after:absolute \
    after:bottom-0 after:left-0 \
    after:w-full after:h-[3px] \
    after:bg-[var(--color-primary)] \
    after:origin-left after:scale-x-0 \
    after:transition-transform \
    after:duration-300 \
    hover:after:scale-x-100";

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
        div { class: CARD_CLS,
            div {
                class: "flex justify-between \
                    items-center gap-2",
                h4 {
                    class: "text-[1.25rem] font-bold \
                        font-[var(--principal-font-family)] \
                        text-[var(--color-text-primary)] \
                        m-0 tracking-tight",
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
            div { class: "flex gap-3 mt-auto pt-2",
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
