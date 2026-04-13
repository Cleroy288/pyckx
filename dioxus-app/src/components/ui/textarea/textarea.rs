//! Textarea — Multi-line text input with label

use dioxus::prelude::*;

/// Base textarea classes
const TEXTAREA_CLS: &str = "\
    w-full py-4 px-6 text-base \
    font-[var(--principal-font-family)] \
    border-2 border-[var(--color-border)] \
    rounded-[12px] bg-[var(--card)] \
    text-[var(--color-text-primary)] \
    resize-y min-h-[80px] \
    transition-all duration-200 \
    outline-none box-border \
    placeholder:text-[var(--color-text-secondary)] \
    focus:border-[var(--color-primary)] \
    disabled:opacity-60 disabled:cursor-not-allowed";

/// Error-state override
const TEXTAREA_ERR: &str = "\
    !border-[var(--color-error)] \
    focus:!shadow-[0_0_0_3px_color-mix(in_srgb,var(--destructive)_10%,transparent)]";

/// Multi-line text input component
#[component]
pub fn Textarea(
    /// Textarea id attribute
    id: String,
    /// Optional label above the textarea
    #[props(default)]
    label: Option<String>,
    /// Placeholder text
    #[props(default)]
    placeholder: String,
    /// Reactive value binding
    value: Signal<String>,
    /// Callback on each input event
    on_input: EventHandler<String>,
    /// Number of visible rows
    #[props(default = 3)]
    rows: u32,
    /// Required flag
    #[props(default = false)]
    required: bool,
    /// Error message signal
    #[props(default = Signal::new(None))]
    error: Signal<Option<String>>,
    /// Extra CSS class
    #[props(default)]
    class: String,
    /// Disabled state
    #[props(default = Signal::new(false))]
    disabled: Signal<bool>,
) -> Element {
    let has_err = error.read().is_some();
    let field_cls =
        format!("flex flex-col gap-2 {}", class);
    let ta_cls = format!(
        "{} {}",
        TEXTAREA_CLS,
        if has_err { TEXTAREA_ERR } else { "" },
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
            textarea {
                id: "{id}",
                class: "{ta_cls}",
                placeholder: "{placeholder}",
                rows: "{rows}",
                required: required,
                disabled: "{disabled}",
                value: "{value}",
                oninput: move |ev: Event<FormData>| {
                    on_input.call(ev.value());
                },
            }
            if let Some(msg) = error.read().as_ref() {
                span {
                    class: "text-xs \
                        text-[var(--color-error)]",
                    "{msg}"
                }
            }
        }
    }
}
