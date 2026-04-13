//! Form controls — Input, Textarea, Select, FormField.

use dioxus::prelude::*;

/// A single option for the `Select` dropdown.
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    /// Build a new `SelectOption`.
    pub fn new(
        value: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self { value: value.into(), label: label.into() }
    }
}

/// Labeled text input with optional error message.
#[component]
pub fn Input(
    input_type: String,
    id: String,
    #[props(default)] label: Option<String>,
    #[props(default)] placeholder: String,
    value: Signal<String>,
    on_input: EventHandler<String>,
    #[props(default = false)] required: bool,
    #[props(default)] minlength: Option<u32>,
    #[props(default = Signal::new(None))] error: Signal<Option<String>>,
    #[props(default)] class: String,
) -> Element {
    let has_err = error.read().is_some();
    let field_cls = format!("flex flex-col gap-2 {}", class);
    let input_cls = format!(
        "{INPUT_CLS} {}",
        if has_err { INPUT_ERR } else { "" },
    );
    rsx! {
        div { class: "{field_cls}",
            {render_label(label.as_deref(), &id)}
            input {
                r#type: "{input_type}",
                id: "{id}",
                class: "{input_cls}",
                placeholder: "{placeholder}",
                required: required,
                minlength: minlength.unwrap_or(0) as i64,
                value: "{value}",
                oninput: move |ev: Event<FormData>| on_input.call(ev.value()),
            }
            {render_error(&error)}
        }
    }
}

/// Multi-line text input with label and optional error.
#[component]
pub fn Textarea(
    id: String,
    #[props(default)] label: Option<String>,
    #[props(default)] placeholder: String,
    value: Signal<String>,
    on_input: EventHandler<String>,
    #[props(default = 3)] rows: u32,
    #[props(default = false)] required: bool,
    #[props(default = Signal::new(None))] error: Signal<Option<String>>,
    #[props(default)] class: String,
    #[props(default = Signal::new(false))] disabled: Signal<bool>,
) -> Element {
    let has_err = error.read().is_some();
    let field_cls = format!("flex flex-col gap-2 {}", class);
    let ta_cls = format!(
        "{TEXTAREA_CLS} {}",
        if has_err { TEXTAREA_ERR } else { "" },
    );
    rsx! {
        div { class: "{field_cls}",
            {render_label(label.as_deref(), &id)}
            textarea {
                id: "{id}",
                class: "{ta_cls}",
                placeholder: "{placeholder}",
                rows: "{rows}",
                required: required,
                disabled: "{disabled}",
                value: "{value}",
                oninput: move |ev: Event<FormData>| on_input.call(ev.value()),
            }
            {render_error(&error)}
        }
    }
}

/// Dropdown select with optional label and placeholder.
#[component]
pub fn Select(
    id: String,
    #[props(default)] label: Option<String>,
    #[props(default)] placeholder: Option<String>,
    value: Signal<String>,
    on_change: EventHandler<String>,
    options: Vec<SelectOption>,
    #[props(default)] class: String,
    #[props(default = Signal::new(false))] disabled: Signal<bool>,
) -> Element {
    let field_cls = format!("flex flex-col gap-2 {}", class);
    rsx! {
        div { class: "{field_cls}",
            {render_label(label.as_deref(), &id)}
            select {
                id: "{id}",
                class: "{SELECT_CLS}",
                disabled: "{disabled}",
                value: "{value}",
                onchange: move |ev: Event<FormData>| on_change.call(ev.value()),
                if let Some(ph) = &placeholder {
                    option { value: "", disabled: true, "{ph}" }
                }
                for opt in options.iter() {
                    option {
                        key: "{opt.value}",
                        value: "{opt.value}",
                        selected: opt.value == *value.read(),
                        "{opt.label}"
                    }
                }
            }
        }
    }
}

/// Label + child input(s) + optional error message.
#[component]
pub fn FormField(
    label: String,
    #[props(default = Signal::new(None))] error: Signal<Option<String>>,
    children: Element,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1.5 {class}",
            label { class: "{FORM_LABEL_CLS}", "{label}" }
            {children}
            {render_error(&error)}
        }
    }
}

/// Render an optional `<label for=id>` above the control.
fn render_label(text: Option<&str>, id: &str) -> Element {
    match text {
        Some(lbl) => rsx! {
            label {
                r#for: "{id}",
                class: "text-sm font-medium \
                    text-[var(--color-text-secondary)] mb-1",
                "{lbl}"
            }
        },
        None => rsx! {},
    }
}

/// Render an optional error message below the control.
fn render_error(error: &Signal<Option<String>>) -> Element {
    match error.read().as_ref() {
        Some(msg) => rsx! {
            span {
                class: "text-[var(--color-error)] text-sm mt-1",
                "{msg}"
            }
        },
        None => rsx! {},
    }
}

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

const INPUT_ERR: &str = "\
    !border-[var(--color-error)] \
    !bg-[color-mix(in_srgb,var(--destructive)_5%,transparent)] \
    focus:!shadow-[0_0_0_3px_color-mix(in_srgb,var(--destructive)_10%,transparent)]";

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

const TEXTAREA_ERR: &str = "\
    !border-[var(--color-error)] \
    focus:!shadow-[0_0_0_3px_color-mix(in_srgb,var(--destructive)_10%,transparent)]";

const SELECT_CLS: &str = "\
    w-full py-4 px-6 pr-10 text-base \
    font-[var(--principal-font-family)] \
    border-2 border-[var(--color-border)] \
    rounded-[12px] bg-[var(--card)] \
    text-[var(--color-text-primary)] \
    cursor-pointer appearance-none \
    transition-all duration-200 \
    outline-none box-border \
    focus:border-[var(--color-primary)] \
    disabled:opacity-60 disabled:cursor-not-allowed";

const FORM_LABEL_CLS: &str = "\
    text-[0.8125rem] font-medium \
    text-[var(--color-text-secondary,#999)]";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_new_sets_fields() {
        let opt = SelectOption::new("val", "Label");
        assert_eq!(opt.value, "val");
    }

    #[test]
    fn test_select_option_new_label_matches() {
        let opt = SelectOption::new("v", "My Label");
        assert_eq!(opt.label, "My Label");
    }

    #[test]
    fn test_select_option_new_accepts_string() {
        let opt = SelectOption::new(
            String::from("k"),
            String::from("K"),
        );
        assert_eq!(opt.value, "k");
    }
}
