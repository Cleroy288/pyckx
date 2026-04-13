//! CtaButton — ink or full CTA with highlight text

use dioxus::prelude::*;

/// Visual variant for the CTA button
#[derive(Clone, Copy, Default, PartialEq)]
pub enum CtaVariant {
    /// Standard nav/hero CTA (btn-ink)
    #[default]
    Ink,
    /// Wide pricing CTA (btn-cta)
    Full,
}

/// Call-to-action button with a lime highlight
#[component]
pub fn CtaButton(
    label: &'static str,
    highlight: &'static str,
    to: &'static str,
    #[props(default)] variant: CtaVariant,
) -> Element {
    let class = match variant {
        CtaVariant::Ink => "btn-ink",
        CtaVariant::Full => "btn-cta",
    };
    rsx! {
        Link { to: to,
            span { class: class,
                "{label}"
                span { class: "cta-hi",
                    "{highlight}"
                }
            }
        }
    }
}
