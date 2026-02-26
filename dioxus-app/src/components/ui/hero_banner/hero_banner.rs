//! HeroBanner — Glass hero section with grid overlay

use dioxus::prelude::*;

/// Banner container classes.
/// Grid overlay and glow accent are handled via
/// global CSS pseudo-elements on `.hero-banner`.
const BANNER_CLS: &str = "\
    hero-banner \
    relative overflow-hidden mb-8 \
    py-12 px-8 text-center \
    bg-[var(--glass-bg)] \
    backdrop-blur-xl \
    border border-[var(--color-border)] \
    border-t-[color-mix(in_srgb,var(--primary)_15%,transparent)] \
    border-b-[color-mix(in_srgb,var(--primary)_15%,transparent)] \
    max-md:mb-6 max-md:py-10 max-md:px-6 \
    max-sm:py-8 max-sm:px-4";

/// Title gradient classes
const TITLE_CLS: &str = "\
    relative z-[1] m-0 mb-2 \
    text-[clamp(2rem,5vw,3rem)] \
    font-bold tracking-tight \
    bg-gradient-to-br \
    from-[var(--color-primary)] \
    to-[color-mix(in_srgb,var(--primary)_70%,black)] \
    bg-clip-text text-transparent";

/// Glass hero banner with optional gradient title
#[component]
pub fn HeroBanner(
    /// Gradient title text (omit for custom)
    #[props(default)]
    title: Option<String>,
    /// Body content (subtitle, buttons, etc.)
    children: Element,
) -> Element {
    rsx! {
        header { class: "{BANNER_CLS}",
            if let Some(t) = &title {
                h1 { class: "{TITLE_CLS}", "{t}" }
            }
            div { class: "z-[1]",
                {children}
            }
        }
    }
}
