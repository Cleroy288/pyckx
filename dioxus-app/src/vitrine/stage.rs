//! `AnimationStage` — canvas particle animation with hub overlay.

use dioxus::prelude::*;

use crate::home::SPLIT_LINE_SVG;

/// Inline JS animation script injected at mount.
const CANVAS_SCRIPT: &str = include_str!("canvas_anim.js");

/// Canvas animation stage with a central "PYCKX" hub.
#[component]
pub fn AnimationStage() -> Element {
    use_effect(move || {
        let _ = js_sys::eval(CANVAS_SCRIPT);
    });
    rsx! {
        div { class: "stage-wrap", id: "vt-stage",
            canvas { id: "vt-canvas" }
            { center_hub() }
        }
    }
}

/// Central hub overlay: ring, split "P" icon, label.
fn center_hub() -> Element {
    rsx! {
        div { class: "center-hub",
            div { class: "hub-ring",
                div { class: "split-icon-hub",
                    span { class: "split-top", "P" }
                    span { class: "split-bottom", "P" }
                    div { dangerous_inner_html: SPLIT_LINE_SVG }
                }
            }
            div { class: "hub-label", "PYCKX" }
        }
    }
}
