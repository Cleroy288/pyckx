//! Logo — hexagon with "P" inside and "PYCKX" text

use super::logo_icon::LogoIcon;
use dioxus::prelude::*;

/// Top-bar link classes
const LINK_CLS: &str = "absolute top-4 left-5 z-30 \
    inline-flex items-center gap-2 \
    px-2.5 py-1.5 rounded-sm \
    cursor-pointer no-underline \
    transition-opacity duration-200 \
    hover:opacity-80 \
    focus:outline-none \
    focus:shadow-[0_0_0_2px_var(--color-primary)]";

/// Brand name classes
const BRAND_CLS: &str = "text-lg font-semibold \
    font-[var(--principal-font-family)] \
    tracking-tight \
    text-[var(--color-text-primary)]";

/// Pyckx logo with hexagon icon and brand name
#[component]
pub fn Logo() -> Element {
    rsx! {
        a {
            href: "/",
            class: "{LINK_CLS}",
            tabindex: 0,
            "aria-label": "Pyckx Home",
            LogoIcon { size: "2rem".to_string() }
            span { class: "{BRAND_CLS}", "PYCKX" }
        }
    }
}
