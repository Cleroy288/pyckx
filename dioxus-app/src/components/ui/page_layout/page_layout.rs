//! PageLayout — Shared page container with side lines

use dioxus::prelude::*;

/// Page container classes.
/// Side lines are handled via global CSS since
/// Tailwind cannot express ::before/::after with
/// calc-based positioning in utility classes alone.
/// The side-line pseudo-elements are defined in
/// the global stylesheet as `.page-layout::before`
/// and `.page-layout::after`.
const PAGE_CLS: &str = "\
    page-layout \
    relative z-[1] flex flex-col \
    w-full min-h-screen \
    pt-[100px] px-8 pb-16 \
    bg-[var(--color-background)] \
    max-md:pt-20 max-md:px-6 max-md:pb-12 \
    max-sm:pt-[70px] max-sm:px-4 max-sm:pb-8";

/// Content wrapper classes
const CONTENT_CLS: &str = "\
    relative flex-1 w-full \
    max-w-[1200px] mx-auto";

/// Full-height page wrapper with centered content
#[component]
pub fn PageLayout(children: Element) -> Element {
    rsx! {
        main { class: "{PAGE_CLS}",
            div { class: "{CONTENT_CLS}",
                {children}
            }
        }
    }
}
