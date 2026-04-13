//! Renders generated course content blocks

use super::block_renderers;
use crate::domain::course_types::{
    ContentBlock, CourseModule, GeneratedCourse,
};
use dioxus::prelude::*;

/// Dispatch a content block to its renderer
fn render_block(block: &ContentBlock) -> Element {
    match block {
        ContentBlock::Title { content } => {
            block_renderers::render_title(content)
        }
        ContentBlock::Subtitle { content } => {
            block_renderers::render_subtitle(content)
        }
        ContentBlock::Text { content } => {
            block_renderers::render_text(content)
        }
        ContentBlock::Schema {
            language,
            content,
        } => {
            block_renderers::render_schema(
                language, content,
            )
        }
        ContentBlock::QcmSet { data } => {
            block_renderers::render_game_card(
                "QCM",
                &data.name,
                &data.description,
            )
        }
        ContentBlock::TrueFalseSet { data } => {
            block_renderers::render_game_card(
                "True/False",
                &data.name,
                &data.description,
            )
        }
        ContentBlock::FlashcardSet { data } => {
            block_renderers::render_game_card(
                "Flashcards",
                &data.name,
                &data.description,
            )
        }
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
                    border-b \
                    border-[var(--color-border)]",
                "{module.title}"
            }
            for (index, block) in module.blocks.iter().enumerate() {
                div { key: "{index}",
                    {render_block(block)}
                }
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
            {render_header(&course)}
            div { class: "flex flex-col gap-8",
                for (index, m) in course.modules.iter().enumerate() {
                    div { key: "{index}",
                        {render_module(m)}
                    }
                }
            }
            div {
                class: "pt-6 border-t-2 \
                    border-[color-mix(in_srgb,var(--primary)_20%,transparent)]",
                {render_module(&course.synthesis)}
            }
        }
    }
}

/// Render the course header section
fn render_header(
    course: &GeneratedCourse,
) -> Element {
    rsx! {
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
    }
}
