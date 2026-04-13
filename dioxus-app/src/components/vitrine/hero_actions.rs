//! HeroActions — flex wrapper for CTA + ghost buttons

use dioxus::prelude::*;

/// Centered row of hero action buttons
#[component]
pub fn HeroActions(children: Element) -> Element {
    rsx! {
        div { class: "hero-actions",
            {children}
        }
    }
}
