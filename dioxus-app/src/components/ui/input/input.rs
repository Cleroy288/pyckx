//! Input — Text input with label and error

use dioxus::prelude::*;

/// Shared input field classes
const INPUT_CLS: &str = "\
    w-full py-4 px-6 text-base \
    font-[var(--principal-font-family)] \
    border-2 border-[var(--color-border)] \
    rounded-[12px] bg-[var(--card)] \
    text-[var(--color-text-primary)] \
    transition-all duration-200 \
    outline-none box-border \
    placeholder:text-[var(--color-text-secondary)] \
    focus:border-[var(--color-primary)]";

/// Error-state input override
const INPUT_ERR: &str = "\
    !border-[var(--color-error)] \
    !bg-[color-mix(in_srgb,var(--destructive)_5%,transparent)] \
    focus:!shadow-[0_0_0_3px_color-mix(in_srgb,var(--destructive)_10%,transparent)]";

/// Reusable text input component
#[component]
pub fn Input(
    /// HTML input type (text, email, password, ...)
    input_type: String,
    /// Input id attribute
    id: String,
    /// Optional label above the input
    #[props(default)]
    label: Option<String>,
    /// Placeholder text
    #[props(default)]
    placeholder: String,
    /// Reactive value binding
    value: Signal<String>,
    /// Callback on each input event
    on_input: EventHandler<String>,
    /// Required flag
    #[props(default = false)]
    required: bool,
    /// Minimum input length
    #[props(default)]
    minlength: Option<u32>,
    /// Error message signal
    #[props(default = Signal::new(None))]
    error: Signal<Option<String>>,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    let has_err = error.read().is_some();
    let field_cls = format!(
        "flex flex-col gap-2 {}",
        class,
    );
    let input_cls = format!(
        "{} {}",
        INPUT_CLS,
        if has_err { INPUT_ERR } else { "" },
    );

    rsx! {
        div { class: "{field_cls}",
            if let Some(lbl) = &label {
                label {
                    r#for: "{id}",
                    class: "text-sm font-medium \
                        text-[var(--color-text-secondary)] \
                        mb-1",
                    "{lbl}"
                }
            }
            input {
                r#type: "{input_type}",
                id: "{id}",
                class: "{input_cls}",
                placeholder: "{placeholder}",
                required: required,
                minlength: minlength
                    .unwrap_or(0) as i64,
                value: "{value}",
                oninput: move |ev: Event<FormData>| {
                    on_input.call(ev.value());
                },
            }
            if let Some(msg) = error.read().as_ref() {
                span {
                    class: "text-[var(--color-error)] \
                        text-sm mt-1",
                    "{msg}"
                }
            }
        }
    }
}
