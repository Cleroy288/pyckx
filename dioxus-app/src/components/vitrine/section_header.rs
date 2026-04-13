//! SectionHeader — two-column heading + description

use dioxus::prelude::*;

/// Section header: h2 title slot + right-side text
#[component]
pub fn SectionHeader(
    title: Element,
    description: &'static str,
) -> Element {
    rsx! {
        div { class: "section-hd",
            h2 { {title} }
            p { "{description}" }
        }
    }
}
