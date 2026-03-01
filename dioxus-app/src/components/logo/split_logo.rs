//! SplitLogo — diagonal-split "PYCKX" hero text

use dioxus::prelude::*;

/// Stacked-rectangles P icon (SVG)
const ICON_SVG: &str = r##"<svg viewBox="0 0 200 200"
  width="100%" height="100%">
  <defs>
    <linearGradient id="split-lg"
      x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6D5AE6"/>
      <stop offset="100%" stop-color="#5B47D6"/>
    </linearGradient>
    <linearGradient id="split-lm"
      x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#2D2A6E"/>
      <stop offset="100%" stop-color="#1E1B50"/>
    </linearGradient>
  </defs>
  <rect x="72" y="36" width="86" height="86"
    rx="22" fill="#DDD8FF" opacity=".55"
    transform="rotate(-14 115 79)"/>
  <rect x="62" y="48" width="86" height="86"
    rx="22" fill="url(#split-lm)"
    transform="rotate(-5 105 91)"/>
  <rect x="36" y="56" width="96" height="96"
    rx="24" fill="url(#split-lg)"/>
  <text x="54" y="130"
    font-family="Arial Black" font-size="70"
    fill="white" fill-opacity=".12"
    stroke="white" stroke-width="2.5">P</text>
  <circle cx="144" cy="62" r="11"
    fill="#7C6FF0"/>
  <circle cx="160" cy="54" r="6"
    fill="#A89DF5" opacity=".75"/>
</svg>"##;

/// Diagonal line overlaid on split text (SVG)
const LINE_SVG: &str = r##"<svg class="split-line"
  viewBox="0 0 100 100"
  preserveAspectRatio="none">
  <line x1="0" y1="46" x2="100" y2="54"/>
</svg>"##;

/// Hero logo: icon + split "PYCKX" text
#[component]
pub fn SplitLogo() -> Element {
    rsx! {
        div { class: "logo-wrap mb-6",
            div {
                class: "shrink-0 \
                    w-[clamp(2.5rem,5vw,4.5rem)] \
                    h-[clamp(2.5rem,5vw,4.5rem)]",
                dangerous_inner_html: ICON_SVG,
            }
            div { class: "split-wrap",
                span { class: "split-top",
                    "PYCKX"
                }
                span { class: "split-bottom",
                    "PYCKX"
                }
                div {
                    dangerous_inner_html: LINE_SVG,
                }
            }
        }
    }
}
