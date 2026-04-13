//! NavLogo — small split "PYCKX" for nav bars

use super::SPLIT_LINE_SVG;
use dioxus::prelude::*;

/// Small split-text PYCKX logo for navigation bars
#[component]
pub fn NavLogo() -> Element {
    rsx! {
        div { class: "split-wrap-sm",
            span { class: "split-top", "PYCKX" }
            span { class: "split-bottom", "PYCKX" }
            div {
                dangerous_inner_html:
                    SPLIT_LINE_SVG,
            }
        }
    }
}
