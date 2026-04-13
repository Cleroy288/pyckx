//! Shared pieces used by both auth pages.

use dioxus::prelude::*;

const CARD_CLS: &str = "\
    max-w-[420px] w-full mx-auto \
    p-8 rounded-[16px] \
    border-2 border-[var(--foreground)] \
    bg-[var(--card)] \
    shadow-[4px_4px_0_var(--foreground)]";

const INPUT_CLS: &str = "\
    w-full py-3 px-4 text-base \
    border-2 border-[var(--color-border)] \
    rounded-[10px] bg-[var(--background)] \
    text-[var(--color-text-primary)] \
    outline-none box-border transition-colors \
    focus:border-[var(--color-primary)]";

const SUBMIT_CLS: &str = "\
    w-full py-3 px-6 text-base font-bold \
    border-2 border-[var(--foreground)] \
    rounded-[10px] cursor-pointer \
    bg-[var(--primary)] \
    text-[var(--primary-foreground)] \
    shadow-[3px_3px_0_var(--foreground)] \
    transition-transform duration-150 \
    active:translate-y-[1px] \
    disabled:opacity-60 disabled:cursor-not-allowed";

const ERROR_CLS: &str = "\
    py-3 px-4 mb-4 rounded-[10px] \
    border-2 border-[var(--destructive)] \
    bg-[color-mix(in_srgb,var(--destructive)_8%,transparent)] \
    text-[var(--destructive)] text-sm";

/// Page shell: centered card with title, subtitle,
/// optional error banner, and a body slot.
#[component]
pub fn AuthCard(
    title: &'static str,
    subtitle: &'static str,
    error: ReadSignal<Option<String>>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "min-h-screen flex items-center \
            justify-center px-4 py-12",
            div { class: "{CARD_CLS}",
                h1 { class: "text-2xl font-bold mb-1",
                    "{title}" }
                p { class: "text-sm text-[var(--color-text-secondary)] \
                    mb-6", "{subtitle}" }
                if let Some(message) = error() {
                    div { class: "{ERROR_CLS}", "{message}" }
                }
                {children}
            }
        }
    }
}

/// Two-way bound text input styled for auth forms.
#[component]
pub fn AuthField(
    input_type: &'static str,
    placeholder: &'static str,
    mut value: Signal<String>,
) -> Element {
    rsx! {
        input {
            class: "{INPUT_CLS}",
            r#type: input_type,
            placeholder: placeholder,
            required: true,
            value: "{value}",
            oninput: move |ev| value.set(ev.value()),
        }
    }
}

/// Full-width primary submit button.
#[component]
pub fn AuthSubmit(
    label: &'static str,
    disabled: bool,
) -> Element {
    rsx! {
        button {
            class: "{SUBMIT_CLS}",
            r#type: "submit",
            disabled: disabled,
            "{label}"
        }
    }
}
