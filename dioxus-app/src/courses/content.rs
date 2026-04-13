//! `ContentRenderer` — paints a full `GeneratedCourse`
//! top-to-bottom: header, modules, synthesis.

use dioxus::prelude::*;

use super::blocks::{block_label, render_block};
use super::types::{CourseModule, GeneratedCourse};

/// CSS class for the course header wrapper
const HEADER_CLS: &str = "flex flex-col gap-2 pb-6 \
    border-b border-[var(--color-border)]";

/// CSS class for the course title (h1)
const TITLE_CLS: &str = "text-2xl font-bold \
    text-[var(--color-text-primary)] m-0";

/// CSS class for the course description paragraph
const DESC_CLS: &str = "text-[0.95rem] \
    text-[var(--color-text-secondary)] \
    m-0 leading-relaxed";

/// CSS class for the course level label
const LEVEL_CLS: &str = "text-[0.8rem] \
    text-[var(--color-text-secondary)] uppercase";

/// CSS class for the module title (h2)
const MODULE_TITLE_CLS: &str = "text-xl font-semibold \
    text-[var(--color-text-primary)] m-0 pb-2 \
    border-b border-[var(--color-border)]";

/// CSS class for the synthesis wrapper
const SYNTHESIS_CLS: &str = "pt-6 border-t-2 \
    border-[color-mix(in_srgb,var(--primary)_20%,transparent)]";

/// Render full generated course content.
#[component]
pub fn ContentRenderer(
    /// Course content to render
    course: GeneratedCourse,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            {render_header(&course)}
            div { class: "flex flex-col gap-8",
                for (index, module) in
                    course.modules.iter().enumerate()
                {
                    div { key: "{index}",
                        {render_module(module)}
                    }
                }
            }
            div { class: SYNTHESIS_CLS,
                {render_module(&course.synthesis)}
            }
        }
    }
}

/// Render the course metadata block at the top.
fn render_header(
    course: &GeneratedCourse,
) -> Element {
    rsx! {
        header { class: HEADER_CLS,
            h1 { class: TITLE_CLS,
                "{course.course_metadata.title}"
            }
            p { class: DESC_CLS,
                "{course.course_metadata.description}"
            }
            span { class: LEVEL_CLS,
                "{course.course_metadata.level}"
            }
        }
    }
}

/// Render a single module with its blocks.
fn render_module(
    module: &CourseModule,
) -> Element {
    rsx! {
        section { class: "flex flex-col gap-4",
            h2 { class: MODULE_TITLE_CLS,
                "{module.title}"
            }
            for (index, block) in
                module.blocks.iter().enumerate()
            {
                div {
                    key: "{index}-{block_label(block)}",
                    {render_block(block)}
                }
            }
        }
    }
}
