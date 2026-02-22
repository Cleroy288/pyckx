// ** content_renderer.rs **
// ==> Renders generated course content blocks

use crate::domain::course_types::{
    ContentBlock, CourseModule, GeneratedCourse,
};
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/intello/course/content_renderer.module.css"
);

/// Render a single content block
fn render_block(block: &ContentBlock) -> impl IntoView {
    match block {
        ContentBlock::Title { content } => view! {
            <h2 class=style::block_title>
                {content.clone()}
            </h2>
        }
        .into_any(),
        ContentBlock::Subtitle { content } => view! {
            <h3 class=style::block_subtitle>
                {content.clone()}
            </h3>
        }
        .into_any(),
        ContentBlock::Text { content } => view! {
            <p class=style::block_text>
                {content.clone()}
            </p>
        }
        .into_any(),
        ContentBlock::Schema {
            language,
            content,
        } => view! {
            <pre class=style::block_code>
                <code>{format!(
                    "// {}\n{}", language, content
                )}</code>
            </pre>
        }
        .into_any(),
        ContentBlock::QcmSet { data } => view! {
            <div class=style::embedded_game>
                <h4>"QCM: " {data.name.clone()}</h4>
                <p>{data.description.clone()}</p>
            </div>
        }
        .into_any(),
        ContentBlock::TrueFalseSet { data } => view! {
            <div class=style::embedded_game>
                <h4>"True/False: " {data.name.clone()}</h4>
                <p>{data.description.clone()}</p>
            </div>
        }
        .into_any(),
        ContentBlock::FlashcardSet { data } => view! {
            <div class=style::embedded_game>
                <h4>"Flashcards: " {data.name.clone()}</h4>
                <p>{data.description.clone()}</p>
            </div>
        }
        .into_any(),
    }
}

/// Render a course module
fn render_module(
    module: &CourseModule,
) -> impl IntoView {
    let title = module.title.clone();
    let blocks = module.blocks.clone();

    view! {
        <section class=style::module>
            <h2 class=style::module_title>{title}</h2>
            {blocks.iter().map(render_block).collect_view()}
        </section>
    }
}

/// Renders full generated course content
#[component]
pub fn ContentRenderer(
    /// Course content to render
    course: GeneratedCourse,
) -> impl IntoView {
    let meta = course.course_metadata;
    let modules = course.modules;
    let synthesis = course.synthesis;

    view! {
        <div class=style::container>
            <header class=style::header>
                <h1 class=style::title>{meta.title}</h1>
                <p class=style::description>
                    {meta.description}
                </p>
                <span class=style::level>{meta.level}</span>
            </header>
            <div class=style::modules>
                {modules.iter().map(render_module)
                    .collect_view()}
            </div>
            <div class=style::synthesis>
                {render_module(&synthesis)}
            </div>
        </div>
    }
}
