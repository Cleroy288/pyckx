//! Pyckx logos used by the home feature.
//!
//! `SPLIT_LINE_SVG` draws a thin diagonal line overlaid on
//! the two-tone "PYCKX" wordmark; `NavLogo` is the small
//! nav-bar variant, `SplitLogo` the hero-sized one.

use dioxus::prelude::*;

/// Diagonal line rendered on top of the split wordmark.
pub const SPLIT_LINE_SVG: &str = r##"<svg
  class="split-line"
  viewBox="0 0 100 100"
  preserveAspectRatio="none">
  <line x1="0" y1="46" x2="100" y2="54"/>
</svg>"##;

/// Small split wordmark for the navigation bar.
#[component]
pub fn NavLogo() -> Element {
    rsx! {
        div { class: "split-wrap-sm",
            span { class: "split-top", "PYCKX" }
            span { class: "split-bottom", "PYCKX" }
            div { dangerous_inner_html: SPLIT_LINE_SVG }
        }
    }
}

/// Hero-sized split logo: split "P" icon + wordmark.
#[component]
pub fn SplitLogo() -> Element {
    rsx! {
        div { class: "logo-wrap mb-6",
            SplitIcon {}
            SplitWord {}
        }
    }
}

/// Big "P" split icon used inside [`SplitLogo`].
#[component]
fn SplitIcon() -> Element {
    rsx! {
        div { class: "split-icon-lg",
            span { class: "split-top", "P" }
            span { class: "split-bottom", "P" }
            div { dangerous_inner_html: SPLIT_LINE_SVG }
        }
    }
}

/// Hero-sized "PYCKX" wordmark used inside [`SplitLogo`].
#[component]
fn SplitWord() -> Element {
    rsx! {
        div { class: "split-wrap",
            span { class: "split-top", "PYCKX" }
            span { class: "split-bottom", "PYCKX" }
            div { dangerous_inner_html: SPLIT_LINE_SVG }
        }
    }
}
