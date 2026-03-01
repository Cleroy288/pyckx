//! LogoIcon — standalone hexagon with "P" inside

use dioxus::prelude::*;

/// Hexagon SVG path data
const HEX: &str = "\
    M21 16V8a2 2 0 0 0-1-1.73\
    l-7-4a2 2 0 0 0-2 0l-7 4\
    A2 2 0 0 0 3 8v8\
    a2 2 0 0 0 1 1.73l7 4\
    a2 2 0 0 0 2 0l7-4\
    A2 2 0 0 0 21 16z";

/// Reusable hexagon icon with "P" letter
#[component]
pub fn LogoIcon(
    /// CSS size (e.g. "2rem", "4rem")
    #[props(default = "2rem".to_string())]
    size: String,
) -> Element {
    let font = format!(
        "calc({} * 0.3125)",
        &size
    );

    rsx! {
        div {
            class: "relative inline-flex \
                items-center justify-center",
            style: "width:{size};height:{size}",
            dangerous_inner_html: format!(
                r#"<svg viewBox="0 0 24 24"
                    fill="none" stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="w-full h-full
                        text-[var(--color-primary)]"
                    xmlns="http://www.w3.org/2000/svg">
                    <path d="{HEX}"/>
                </svg>
                <span class="absolute font-bold
                    font-[var(--principal-font-family)]
                    text-[var(--color-primary)]
                    select-none"
                    style="font-size:{font}">
                    P
                </span>"#
            ),
        }
    }
}
