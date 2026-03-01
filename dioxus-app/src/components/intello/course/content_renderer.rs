// ** content_renderer.rs **
// ==> Renders generated course content blocks

use crate::domain::course_types::{
    ContentBlock, CourseModule, GeneratedCourse,
};
use dioxus::prelude::*;

/// Render a single content block
fn render_block(block: &ContentBlock) -> Element {
    match block {
        ContentBlock::Title { content } => rsx! {
            h2 {
                class: "text-[1.1rem] font-semibold \
                    text-[var(--color-text-primary)] \
                    mt-2 mb-0",
                "{content}"
            }
        },
        ContentBlock::Subtitle { content } => rsx! {
            h3 {
                class: "text-base font-medium \
                    text-[var(--color-text-secondary)] \
                    mt-1 mb-0",
                "{content}"
            }
        },
        ContentBlock::Text { content } => rsx! {
            p {
                class: "text-[0.9rem] \
                    text-[var(--color-text-secondary)] \
                    leading-relaxed m-0",
                "{content}"
            }
        },
        ContentBlock::Schema {
            language,
            content,
        } => rsx! {
            pre {
                class: "bg-black/30 \
                    border border-[var(--color-border)] \
                    p-4 overflow-x-auto \
                    text-[0.85rem] \
                    text-[var(--color-text-primary)] \
                    m-0",
                code {
                    "// {language}\n{content}"
                }
            }
        },
        ContentBlock::QcmSet { data } => rsx! {
            div {
                class: "p-4 border border-dashed \
                    border-[color-mix(in_srgb,var(--primary)_30%,transparent)] \
                    bg-[color-mix(in_srgb,var(--primary)_5%,transparent)]",
                h4 {
                    class: "text-[0.95rem] \
                        text-[var(--color-text-secondary)] \
                        m-0 mb-1",
                    "QCM: {data.name}"
                }
                p {
                    class: "text-[0.85rem] \
                        text-[var(--color-text-secondary)] \
                        m-0",
                    "{data.description}"
                }
            }
        },
        ContentBlock::TrueFalseSet { data } => rsx! {
            div {
                class: "p-4 border border-dashed \
                    border-[color-mix(in_srgb,var(--primary)_30%,transparent)] \
                    bg-[color-mix(in_srgb,var(--primary)_5%,transparent)]",
                h4 {
                    class: "text-[0.95rem] \
                        text-[var(--color-text-secondary)] \
                        m-0 mb-1",
                    "True/False: {data.name}"
                }
                p {
                    class: "text-[0.85rem] \
                        text-[var(--color-text-secondary)] \
                        m-0",
                    "{data.description}"
                }
            }
        },
        ContentBlock::FlashcardSet { data } => rsx! {
            div {
                class: "p-4 border border-dashed \
                    border-[color-mix(in_srgb,var(--primary)_30%,transparent)] \
                    bg-[color-mix(in_srgb,var(--primary)_5%,transparent)]",
                h4 {
                    class: "text-[0.95rem] \
                        text-[var(--color-text-secondary)] \
                        m-0 mb-1",
                    "Flashcards: {data.name}"
                }
                p {
                    class: "text-[0.85rem] \
                        text-[var(--color-text-secondary)] \
                        m-0",
                    "{data.description}"
                }
            }
        },
    }
}

/// Render a course module
fn render_module(
    module: &CourseModule,
) -> Element {
    rsx! {
        section { class: "flex flex-col gap-4",
            h2 {
                class: "text-xl font-semibold \
                    text-[var(--color-text-primary)] \
                    m-0 pb-2 \
                    border-b border-[var(--color-border)]",
                "{module.title}"
            }
            for block in module.blocks.iter() {
                {render_block(block)}
            }
        }
    }
}

/// Renders full generated course content
#[component]
pub fn ContentRenderer(
    /// Course content to render
    course: GeneratedCourse,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-8",
            // Header
            header {
                class: "flex flex-col gap-2 pb-6 \
                    border-b \
                    border-[var(--color-border)]",
                h1 {
                    class: "text-2xl font-bold \
                        text-[var(--color-text-primary)] \
                        m-0",
                    "{course.course_metadata.title}"
                }
                p {
                    class: "text-[0.95rem] \
                        text-[var(--color-text-secondary)] \
                        m-0 leading-relaxed",
                    "{course.course_metadata.description}"
                }
                span {
                    class: "text-[0.8rem] \
                        text-[var(--color-text-secondary)] \
                        uppercase",
                    "{course.course_metadata.level}"
                }
            }
            // Modules
            div { class: "flex flex-col gap-8",
                for m in course.modules.iter() {
                    {render_module(m)}
                }
            }
            // Synthesis
            div {
                class: "pt-6 \
                    border-t-2 \
                    border-[color-mix(in_srgb,var(--primary)_20%,transparent)]",
                {render_module(&course.synthesis)}
            }
        }
    }
}
