//! Modal dialogs — Modal (generic) and ConfirmDialog.

use super::button::{Button, ButtonVariant};
use super::icon::Icon;
use dioxus::prelude::*;

/// Overlay dialog rendered when `open` is true.
#[component]
pub fn Modal(
    open: Signal<bool>,
    #[props(default)] on_close: EventHandler<()>,
    #[props(default)] title: Option<String>,
    children: Element,
) -> Element {
    if !*open.read() {
        return rsx! {};
    }
    rsx! {
        div {
            class: "{BACKDROP_CLS}",
            onclick: move |_| on_close.call(()),
            div {
                class: "{MODAL_CLS}",
                onclick: move |ev| ev.stop_propagation(),
                div { class: "{HEADER_CLS}",
                    {render_title(title.as_deref())}
                    button {
                        class: "{CLOSE_CLS}",
                        onclick: move |_| on_close.call(()),
                        Icon { icon_name: "X".to_string() }
                    }
                }
                div { class: "p-0", {children} }
            }
        }
    }
}

/// "Are you sure?" dialog with Cancel and Confirm actions.
#[component]
pub fn ConfirmDialog(
    open: Signal<bool>,
    #[props(default = "Confirm".to_string())] title: String,
    #[props(default = "Are you sure?".to_string())] message: String,
    #[props(default = "Delete".to_string())] confirm_label: String,
    on_confirm: EventHandler<()>,
) -> Element {
    let mut close = move || open.set(false);
    rsx! {
        Modal {
            open: open,
            title: title,
            on_close: move |_| close(),
            div { class: "{BODY_CLS}", "{message}" }
            div { class: "{ACTIONS_CLS}",
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

/// Render the gradient modal title when set.
fn render_title(text: Option<&str>) -> Element {
    match text {
        Some(title) => rsx! {
            h2 { class: "{TITLE_CLS}", "{title}" }
        },
        None => rsx! {},
    }
}

const BACKDROP_CLS: &str = "\
    fixed inset-0 z-[1000] \
    flex items-center justify-center \
    bg-black/60 animate-[fadeIn_0.2s_ease]";

const MODAL_CLS: &str = "\
    relative w-[90%] max-w-[500px] \
    max-h-[85vh] overflow-y-auto \
    bg-[var(--color-background)] \
    border border-[var(--color-border)] \
    shadow-[0_8px_40px_rgba(0,0,0,0.4)] \
    animate-[slideUp_0.25s_ease]";

const HEADER_CLS: &str = "\
    relative overflow-hidden flex \
    items-center justify-center p-6 \
    bg-[var(--glass-bg)] \
    border-b border-[color-mix(in_srgb,var(--primary)_15%,transparent)]";

const TITLE_CLS: &str = "\
    relative z-[1] m-0 text-2xl font-bold \
    tracking-tight \
    bg-gradient-to-br \
    from-[var(--color-primary)] \
    to-[color-mix(in_srgb,var(--primary)_70%,black)] \
    bg-clip-text text-transparent";

const CLOSE_CLS: &str = "\
    absolute right-6 top-1/2 -translate-y-1/2 \
    z-[1] flex items-center justify-center \
    w-8 h-8 p-0 bg-transparent \
    border border-[var(--color-border)] \
    text-[var(--color-text-secondary)] \
    cursor-pointer shrink-0 \
    transition-all duration-[var(--transition-fast)] \
    hover:border-[var(--color-primary)] \
    hover:text-[var(--color-primary)]";

const BODY_CLS: &str = "\
    p-6 text-sm leading-relaxed \
    text-[var(--color-text-secondary)]";

const ACTIONS_CLS: &str = "flex gap-3 justify-end p-6 pt-0";
