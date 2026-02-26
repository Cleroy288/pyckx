//! WelcomeHeader — greeting banner with username

use crate::components::ui::hero_banner::HeroBanner;
use dioxus::prelude::*;

/// Username gradient classes
const NAME_CLS: &str = "bg-gradient-to-r \
    from-[var(--color-primary)] \
    via-[color-mix(in_srgb,var(--primary)_60%,white)] \
    to-[var(--color-primary)] \
    bg-[length:200%_100%] \
    bg-clip-text text-transparent \
    animate-[shimmer_3s_ease-in-out_infinite]";

/// Greeting banner with username
#[component]
pub fn WelcomeHeader(
    /// Reactive username signal
    username: Signal<String>,
    /// Optional subtitle text
    #[props(default = "Select an app to get started".to_string())]
    subtitle: String,
) -> Element {
    rsx! {
        HeroBanner {
            h1 {
                class: "relative z-[1] \
                    text-[clamp(2rem,5vw,3rem)] \
                    font-bold \
                    text-[var(--color-text-primary)] \
                    mb-2 tracking-tight",
                "Welcome, "
                span { class: "{NAME_CLS}", "{username}" }
            }
            p {
                class: "relative z-[1] text-lg \
                    text-[var(--color-text-secondary)] \
                    tracking-wide",
                "{subtitle}"
            }
        }
    }
}
