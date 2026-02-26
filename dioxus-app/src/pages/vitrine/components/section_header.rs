//! Section header component for vitrine page

use dioxus::prelude::*;

/// Centered section header with title and subtitle
#[component]
pub fn SectionHeader(
    /// Section title
    title: String,
    /// Optional subtitle
    #[props(default)]
    subtitle: Option<String>,
) -> Element {
    rsx! {
        div {
            class: "text-center mb-16",
            h2 {
                class: "text-[clamp(2.5rem,5vw,3.5rem)] \
                    font-extrabold \
                    text-[var(--color-text-primary)] \
                    mb-4 tracking-tight",
                "{title}"
            }
            if let Some(sub) = subtitle {
                p {
                    class: "text-[clamp(1rem,2vw,1.25rem)] \
                        text-[var(--color-text-secondary)] \
                        max-w-[600px] mx-auto leading-relaxed",
                    "{sub}"
                }
            }
        }
    }
}
