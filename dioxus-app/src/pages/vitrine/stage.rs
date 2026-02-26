//! AnimationStage — canvas particle animation

use dioxus::prelude::*;

/// JS animation script loaded via include_str
const CANVAS_SCRIPT: &str =
    include_str!("canvas_anim.js");

/// Hub logo SVG (110x110, centered on canvas)
const HUB_LOGO: &str = r##"<svg width="110"
  height="110" viewBox="0 0 200 200">
  <defs>
    <linearGradient id="hub-lg"
      x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6D5AE6"/>
      <stop offset="100%" stop-color="#5B47D6"/>
    </linearGradient>
    <linearGradient id="hub-lm"
      x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#2D2A6E"/>
      <stop offset="100%" stop-color="#1E1B50"/>
    </linearGradient>
  </defs>
  <rect x="72" y="36" width="86" height="86"
    rx="22" fill="#DDD8FF" opacity=".55"
    transform="rotate(-14 115 79)"/>
  <rect x="62" y="48" width="86" height="86"
    rx="22" fill="url(#hub-lm)"
    transform="rotate(-5 105 91)"/>
  <rect x="36" y="56" width="96" height="96"
    rx="24" fill="url(#hub-lg)"/>
  <text x="54" y="130"
    font-family="Arial Black" font-size="70"
    fill="white" fill-opacity=".12"
    stroke="white" stroke-width="2.5">P</text>
  <circle cx="144" cy="62" r="11"
    fill="#7C6FF0"/>
  <circle cx="160" cy="54" r="6"
    fill="#A89DF5" opacity=".75"/>
</svg>"##;

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
                div {
                    class: "hub-ring",
                    dangerous_inner_html: HUB_LOGO,
                }
                div { class: "hub-label", "PYCKX" }
            }
        }
    }
}
