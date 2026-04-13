//! SplitLogo — diagonal-split "PYCKX" hero text

use dioxus::prelude::*;

/// Diagonal line overlaid on split text (SVG)
pub const SPLIT_LINE_SVG: &str = r##"<svg
  class="split-line"
  viewBox="0 0 100 100"
  preserveAspectRatio="none">
  <line x1="0" y1="46" x2="100" y2="54"/>
</svg>"##;

/// Hero logo: split-P icon + split "PYCKX" text
#[component]
pub fn SplitLogo() -> Element {
    rsx! {
        div { class: "logo-wrap mb-6",
            div { class: "split-icon-lg",
                span { class: "split-top", "P" }
                span { class: "split-bottom", "P" }
                div {
                    dangerous_inner_html:
                        SPLIT_LINE_SVG,
                }
            }
            div { class: "split-wrap",
                span { class: "split-top", "PYCKX" }
                span { class: "split-bottom",
                    "PYCKX"
                }
                div {
                    dangerous_inner_html:
                        SPLIT_LINE_SVG,
                }
            }
        }
    }
}
