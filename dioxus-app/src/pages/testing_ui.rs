//! Testing UI — blank sandbox page

use dioxus::prelude::*;

#[component]
fn Rectangle(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: String,
) -> Element {
    rsx! {
        div {
            style: format!(
                "position: absolute; \
                left: {x}px; \
                top: {y}px; \
                width: {width}px; \
                height: {height}px; \
                background-color: {color};"
            ),
        }
    }
}

/// Empty page for UI testing
pub fn TestingUiPage() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-[var(--background)]",
            h1 {
                class: "text-3xl font-bold text-center \
                    pt-20 text-[var(--foreground)]",
                "Testing UI"
            }
                Rectangle {
                    x: 100,
                    y: 200,
                    width: 50,
                    height: 50,
                    color: "rgba(255, 0, 0, 0.5)".to_string(),
                }

                h1 {
                    style: "font-family: 'Unbounded', sans-serif;",
                    class: "text-3xl font-bold text-center \
                        pt-20 text-[var(--foreground)]",
                    "Testing UI"
                }

        }
    }
}
