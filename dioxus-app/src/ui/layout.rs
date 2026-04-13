//! Layout primitives — PageLayout, HeroBanner, SectionDivider.

use crate::home::SPLIT_LINE_SVG;
use dioxus::prelude::*;

/// Full-height page wrapper with centered content.
#[component]
pub fn PageLayout(children: Element) -> Element {
    rsx! {
        main { class: "{PAGE_CLS}",
            div { class: "{CONTENT_CLS}", {children} }
        }
    }
}

/// Hero banner with an optional split-style title.
#[component]
pub fn HeroBanner(
    #[props(default)] title: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        header { class: "{BANNER_CLS}",
            {render_split_title(title.as_deref())}
            div { class: "z-[1]", {children} }
        }
    }
}

/// Horizontal divider with a centered label (line — LABEL — line).
#[component]
pub fn SectionDivider(label: String) -> Element {
    rsx! {
        div { class: "{DIVIDER_CLS}",
            span { class: "{DIVIDER_LINE_CLS}" }
            "{label}"
            span { class: "{DIVIDER_LINE_CLS}" }
        }
    }
}

/// Render the split-title block when a title is set.
fn render_split_title(title: Option<&str>) -> Element {
    match title {
        Some(text) => rsx! {
            div { class: "hero-split-title",
                div { class: "split-wrap",
                    span { class: "split-top", "{text}" }
                    span { class: "split-bottom", "{text}" }
                    div { dangerous_inner_html: SPLIT_LINE_SVG }
                }
            }
        },
        None => rsx! {},
    }
}

const PAGE_CLS: &str = "\
    page-layout \
    relative z-[1] flex flex-col \
    w-full min-h-screen \
    pt-[100px] px-8 pb-16 \
    bg-[var(--color-background)] \
    max-md:pt-20 max-md:px-6 max-md:pb-12 \
    max-sm:pt-[70px] max-sm:px-4 max-sm:pb-8";

const CONTENT_CLS: &str = "\
    relative flex-1 w-full \
    max-w-[1200px] mx-auto";

const BANNER_CLS: &str = "\
    relative overflow-hidden mb-8 \
    py-12 px-8 text-center \
    bg-[var(--card)] \
    border-[1.5px] border-[var(--border)] \
    rounded-[12px] \
    max-md:mb-6 max-md:py-10 max-md:px-6 \
    max-sm:py-8 max-sm:px-4";

const DIVIDER_CLS: &str = "\
    flex items-center gap-3 \
    text-[0.8rem] font-semibold \
    text-[var(--color-primary)] \
    uppercase tracking-wider \
    whitespace-nowrap";

const DIVIDER_LINE_CLS: &str = "\
    flex-1 h-px \
    bg-[color-mix(in_srgb,var(--primary)_25%,transparent)]";
