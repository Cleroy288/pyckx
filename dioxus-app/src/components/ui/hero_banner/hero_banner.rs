//! HeroBanner — hero section with split title

use crate::components::logo::SPLIT_LINE_SVG;
use dioxus::prelude::*;

/// Banner container classes
const BANNER_CLS: &str = "\
    relative overflow-hidden mb-8 \
    py-12 px-8 text-center \
    bg-[var(--card)] \
    border-[1.5px] border-[var(--border)] \
    rounded-[12px] \
    max-md:mb-6 max-md:py-10 max-md:px-6 \
    max-sm:py-8 max-sm:px-4";

/// Hero banner with split-style title (like PYCKX logo)
#[component]
pub fn HeroBanner(
    /// Title text displayed in split style
    #[props(default)]
    title: Option<String>,
    /// Body content (subtitle, buttons, etc.)
    children: Element,
) -> Element {
    rsx! {
        header { class: "{BANNER_CLS}",
            if let Some(t) = &title {
                div { class: "hero-split-title",
                    div { class: "split-wrap",
                        span {
                            class: "split-top",
                            "{t}"
                        }
                        span {
                            class: "split-bottom",
                            "{t}"
                        }
                        div {
                            dangerous_inner_html:
                                SPLIT_LINE_SVG,
                        }
                    }
                }
            }
            div { class: "z-[1]",
                {children}
            }
        }
    }
}
