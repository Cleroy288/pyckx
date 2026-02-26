//! FormField — Label + child input + error wrapper

use dioxus::prelude::*;

/// Form field wrapper with label and error
#[component]
pub fn FormField(
    /// Label text above the field
    label: String,
    /// Optional error message below the field
    #[props(default = Signal::new(None))]
    error: Signal<Option<String>>,
    /// The input element(s) inside
    children: Element,
    /// Extra CSS class
    #[props(default)]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-1.5 {class}",
            label {
                class: "text-[0.8125rem] font-medium \
                    text-[var(--color-text-secondary,#999)]",
                "{label}"
            }
            {children}
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
