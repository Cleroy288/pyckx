//! Select — Dropdown selection with label

use dioxus::prelude::*;

/// Single dropdown option
#[derive(Clone, PartialEq)]
pub struct SelectOption {
    /// Option value attribute
    pub value: String,
    /// Display label
    pub label: String,
}

impl SelectOption {
    /// Create a new select option
    pub fn new(
        value: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Select dropdown classes
const SELECT_CLS: &str = "\
    w-full py-4 px-6 pr-10 text-base \
    font-[var(--principal-font-family)] \
    border-2 border-[var(--color-border)] \
    rounded-[12px] bg-[var(--card)] \
    text-[var(--color-text-primary)] \
    cursor-pointer appearance-none \
    bg-[url('data:image/svg+xml,%3csvg_xmlns=%27http://www.w3.org/2000/svg%27_fill=%27none%27_viewBox=%270_0_20_20%27%3e%3cpath_stroke=%27%236b7280%27_stroke-linecap=%27round%27_stroke-linejoin=%27round%27_stroke-width=%271.5%27_d=%27M6_8l4_4_4-4%27/%3e%3c/svg%3e')] \
    bg-[position:right_0.75rem_center] \
    bg-no-repeat bg-[length:1.25rem] \
    transition-all duration-200 \
    outline-none box-border \
    focus:border-[var(--color-primary)] \
    disabled:opacity-60 disabled:cursor-not-allowed";

/// Dropdown select component
#[component]
pub fn Select(
    /// Select id attribute
    id: String,
    /// Optional label above the select
    #[props(default)]
    label: Option<String>,
    /// Placeholder option text
    #[props(default)]
    placeholder: Option<String>,
    /// Reactive value binding
    value: Signal<String>,
    /// Callback when selection changes
    on_change: EventHandler<String>,
    /// Available options
    options: Vec<SelectOption>,
    /// Extra CSS class
    #[props(default)]
    class: String,
    /// Disabled state
    #[props(default = Signal::new(false))]
    disabled: Signal<bool>,
) -> Element {
    let field_cls =
        format!("flex flex-col gap-2 {}", class);

    rsx! {
        div { class: "{field_cls}",
            if let Some(lbl) = &label {
                label {
                    r#for: "{id}",
                    class: "text-sm font-medium \
                        text-[var(--color-text-secondary)]",
                    "{lbl}"
                }
            }
            select {
                id: "{id}",
                class: "{SELECT_CLS}",
                disabled: "{disabled}",
                value: "{value}",
                onchange: move |ev: Event<FormData>| {
                    on_change.call(ev.value());
                },
                if let Some(ph) = &placeholder {
                    option {
                        value: "",
                        disabled: true,
                        "{ph}"
                    }
                }
                for opt in options.iter() {
                    option {
                        key: "{opt.value}",
                        value: "{opt.value}",
                        selected: opt.value
                            == *value.read(),
                        "{opt.label}"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_option_new_sets_fields() {
        let opt = SelectOption::new("val", "Label");
        assert_eq!(opt.value, "val");
        assert_eq!(opt.label, "Label");
    }

    #[test]
    fn test_select_option_new_with_string() {
        let opt = SelectOption::new(
            String::from("k"),
            String::from("Key"),
        );
        assert_eq!(opt.value, "k");
        assert_eq!(opt.label, "Key");
    }
}
