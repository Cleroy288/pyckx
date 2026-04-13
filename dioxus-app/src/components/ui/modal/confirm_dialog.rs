//! ConfirmDialog — "Are you sure?" modal

use super::Modal;
use crate::components::ui::button::{
    Button, ButtonVariant,
};
use dioxus::prelude::*;

/// Body text classes
const BODY_CLS: &str = "\
    p-6 text-sm leading-relaxed \
    text-[var(--color-text-secondary)]";

/// Actions row classes
const ACTIONS_CLS: &str = "\
    flex gap-3 justify-end p-6 pt-0";

/// Confirmation dialog component
#[component]
pub fn ConfirmDialog(
    /// Whether the dialog is visible
    open: Signal<bool>,
    /// Dialog title
    #[props(default = "Confirm".to_string())]
    title: String,
    /// Message displayed in the body
    #[props(default = "Are you sure?".to_string())]
    message: String,
    /// Confirm button label
    #[props(default = "Delete".to_string())]
    confirm_label: String,
    /// Called when confirmed
    on_confirm: EventHandler<()>,
) -> Element {
    let mut close = move || open.set(false);

    rsx! {
        Modal {
            open: open,
            title: title,
            on_close: move |_| close(),
            div { class: BODY_CLS,
                "{message}"
            }
            div { class: ACTIONS_CLS,
                Button {
                    text: "Cancel",
                    variant: ButtonVariant::Outline,
                    on_click: move |_| close(),
                }
                Button {
                    text: confirm_label,
                    variant: ButtonVariant::Destructive,
                    on_click: move |_| {
                        on_confirm.call(());
                        close();
                    },
                }
            }
        }
    }
}
