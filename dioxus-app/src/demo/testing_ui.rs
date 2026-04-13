//! Testing UI — blank sandbox page for visual experiments.

use dioxus::prelude::*;

/// Absolute rectangle used as a visual marker in the sandbox.
#[component]
fn Rectangle(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: String,
) -> Element {
    let style = format!(
        "position: absolute; left: {x}px; top: {y}px; \
         width: {width}px; height: {height}px; \
         background-color: {color};"
    );
    rsx! { div { style: "{style}" } }
}

/// Blank UI sandbox page for ad-hoc visual testing.
pub fn TestingUiPage() -> Element {
    rsx! {
        div { class: "min-h-screen bg-[var(--background)]",
            {heading("Testing UI", None)}
            Rectangle {
                x: 100, y: 200,
                width: 50, height: 50,
                color: "rgba(255, 0, 0, 0.5)".to_string(),
            }
            {heading("Testing UI", Some("Unbounded"))}
        }
    }
}

/// Centered page heading, optionally forcing a font family.
fn heading(text: &str, font: Option<&str>) -> Element {
    let classes = "text-3xl font-bold text-center pt-20 \
                   text-[var(--foreground)]";
    match font {
        Some(family) => rsx! {
            h1 {
                style: "font-family: '{family}', sans-serif;",
                class: "{classes}",
                "{text}"
            }
        },
        None => rsx! { h1 { class: "{classes}", "{text}" } },
    }
}
