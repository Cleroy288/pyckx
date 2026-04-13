//! TabPanel — conditional content for a tab index

use dioxus::prelude::*;

/// Conditionally renders content for a tab index
#[component]
pub fn TabPanel(
    /// Index this panel corresponds to
    index: usize,
    /// Active tab signal
    active: Signal<usize>,
    /// Panel content
    children: Element,
) -> Element {
    let display = if *active.read() == index {
        "block"
    } else {
        "none"
    };

    rsx! {
        div {
            class: "py-5",
            style: "display:{display}",
            {children}
        }
    }
}
