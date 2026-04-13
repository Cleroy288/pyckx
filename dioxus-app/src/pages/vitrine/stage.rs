//! AnimationStage — canvas particle animation

use crate::components::logo::SPLIT_LINE_SVG;
use dioxus::prelude::*;

/// JS animation script loaded via include_str
const CANVAS_SCRIPT: &str =
    include_str!("canvas_anim.js");

/// Canvas animation stage with hub overlay
#[component]
pub fn AnimationStage() -> Element {
    use_effect(move || {
        let _ = js_sys::eval(CANVAS_SCRIPT);
    });
    rsx! {
        div {
            class: "stage-wrap",
            id: "vt-stage",
            canvas { id: "vt-canvas" }
            div { class: "center-hub",
                div { class: "hub-ring",
                    div { class: "split-icon-hub",
                        span {
                            class: "split-top",
                            "P"
                        }
                        span {
                            class: "split-bottom",
                            "P"
                        }
                        div {
                            dangerous_inner_html:
                                SPLIT_LINE_SVG,
                        }
                    }
                }
                div {
                    class: "hub-label",
                    "PYCKX"
                }
            }
        }
    }
}
