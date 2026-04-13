//! Reusable vitrine buttons and layout primitives.
//!
//! Newspaper order:
//! 1. `CtaButton`     — primary call-to-action (ink / full variants)
//! 2. `GhostButton`   — secondary outlined action (route / anchor)
//! 3. `HeroActions`   — flex row wrapping hero buttons
//! 4. `SectionHeader` — two-column title + description block

use dioxus::prelude::*;


/// Visual variant for the CTA button.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CtaVariant {
    /// Standard nav/hero CTA (`btn-ink`).
    #[default]
    Ink,
    /// Wide pricing CTA (`btn-cta`).
    Full,
}

/// Maps a `CtaVariant` to its CSS class name.
fn cta_class(variant: CtaVariant) -> &'static str {
    match variant {
        CtaVariant::Ink => "btn-ink",
        CtaVariant::Full => "btn-cta",
    }
}

/// Primary call-to-action with a lime highlight span.
#[component]
pub fn CtaButton(
    label: &'static str,
    highlight: &'static str,
    to: &'static str,
    #[props(default)] variant: CtaVariant,
) -> Element {
    let class = cta_class(variant);
    rsx! {
        Link { to: to,
            span { class: class,
                "{label}"
                span { class: "cta-hi", "{highlight}" }
            }
        }
    }
}


/// Destination of a ghost button: internal route or
/// in-page anchor.
#[derive(Clone, PartialEq)]
pub enum GhostTarget {
    /// Internal route — rendered via `Link`.
    Route(&'static str),
    /// Anchor scroll — rendered via `<a href>`.
    Anchor(&'static str),
}

/// Outlined ghost button for secondary actions.
#[component]
pub fn GhostButton(
    label: &'static str,
    target: GhostTarget,
) -> Element {
    match target {
        GhostTarget::Route(to) => rsx! {
            Link { to: to,
                span { class: "btn-ghost", "{label}" }
            }
        },
        GhostTarget::Anchor(href) => rsx! {
            a { href: href, class: "btn-ghost", "{label}" }
        },
    }
}


/// Centered row of hero action buttons.
#[component]
pub fn HeroActions(children: Element) -> Element {
    rsx! {
        div { class: "hero-actions", {children} }
    }
}

/// Section header: `h2` title slot + right-side paragraph.
#[component]
pub fn SectionHeader(
    title: Element,
    description: &'static str,
) -> Element {
    rsx! {
        div { class: "section-hd",
            h2 { {title} }
            p { "{description}" }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cta_class_ink_variant() {
        assert_eq!(cta_class(CtaVariant::Ink), "btn-ink");
    }

    #[test]
    fn test_cta_class_full_variant() {
        assert_eq!(cta_class(CtaVariant::Full), "btn-cta");
    }

    #[test]
    fn test_cta_variant_default_is_ink() {
        assert_eq!(CtaVariant::default(), CtaVariant::Ink);
    }
}
