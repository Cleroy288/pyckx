//! AuthSubmitBtn — full-width submit with arrow

use dioxus::prelude::*;

/// Submit button for auth forms (btn-ink, full width)
#[component]
pub fn AuthSubmitBtn(
    label: &'static str,
    disabled: bool,
) -> Element {
    rsx! {
        button {
            class: "btn-ink w-full justify-center",
            r#type: "submit",
            disabled: disabled,
            "{label}"
            span { class: "cta-hi", " \u{2192}" }
        }
    }
}
