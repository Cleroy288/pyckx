//! AuthInput — styled form input with signal binding

use dioxus::prelude::*;

/// Two-way bound input for auth forms
#[component]
pub fn AuthInput(
    input_type: &'static str,
    placeholder: &'static str,
    mut value: Signal<String>,
) -> Element {
    rsx! {
        input {
            class: "auth-input",
            r#type: input_type,
            placeholder: placeholder,
            required: true,
            value: "{value}",
            oninput: move |ev| {
                value.set(ev.value());
            },
        }
    }
}
