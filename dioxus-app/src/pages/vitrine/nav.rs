//! VitrineNav — sticky navigation bar

use dioxus::prelude::*;

/// Logo SVG for the nav bar (32x32)
const NAV_LOGO: &str = r##"<svg width="94" height="94"
  viewBox="0 0 200 200">
  <defs>
    <linearGradient id="nav-lg"
      x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#6D5AE6"/>
      <stop offset="100%" stop-color="#5B47D6"/>
    </linearGradient>
    <linearGradient id="nav-lm"
      x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="#2D2A6E"/>
      <stop offset="100%" stop-color="#1E1B50"/>
    </linearGradient>
  </defs>
  <rect x="72" y="36" width="86" height="86"
    rx="22" fill="#DDD8FF" opacity=".55"
    transform="rotate(-14 115 79)"/>
  <rect x="62" y="48" width="86" height="86"
    rx="22" fill="url(#nav-lm)"
    transform="rotate(-5 105 91)"/>
  <rect x="36" y="56" width="96" height="96"
    rx="24" fill="url(#nav-lg)"/>
  <text x="54" y="130"
    font-family="Arial Black" font-size="70"
    fill="white" fill-opacity=".12"
    stroke="white" stroke-width="2.5">P</text>
  <circle cx="144" cy="62" r="11"
    fill="#7C6FF0"/>
  <circle cx="160" cy="54" r="6"
    fill="#A89DF5" opacity=".75"/>
</svg>"##;

/// SVG diagonal line for split-text effect
const SPLIT_LINE: &str = r##"<svg class="split-line"
  viewBox="0 0 100 100"
  preserveAspectRatio="none">
  <line x1="0" y1="46" x2="100" y2="54"/>
</svg>"##;

/// Sticky nav bar with logo, links, buttons
#[component]
pub fn VitrineNav() -> Element {
    rsx! {
        nav {
            div { class: "logo-wrap",
                div {
                    dangerous_inner_html: NAV_LOGO,
                }
                div { class: "split-wrap-sm",
                    span {
                        class: "split-top",
                        "PYCKX"
                    }
                    span {
                        class: "split-bottom",
                        "PYCKX"
                    }
                    div {
                        dangerous_inner_html: SPLIT_LINE,
                    }
                }
            }
            div { class: "nav-links",
                a { href: "#services", "Services" }
                a { href: "#pricing", "Tarifs" }
            }
            div {
                style: "display:flex;gap:10px",
                Link { to: "/login",
                    span {
                        class: "btn-ghost",
                        "Login"
                    }
                }
                Link { to: "/login",
                    span {
                        class: "btn-ink",
                        "Commencer \u{2192}"
                    }
                }
            }
        }
    }
}
