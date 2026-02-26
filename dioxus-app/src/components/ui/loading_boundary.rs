//! LoadingBoundary — Spinner while loading

use crate::components::ui::spinner::Spinner;
use dioxus::prelude::*;

/// Shows Spinner while loading, children when done.
/// Uses display toggle to keep children mounted.
#[component]
pub fn LoadingBoundary(
    /// True while data is loading
    loading: Signal<bool>,
    /// Content to show once loaded
    children: Element,
) -> Element {
    let is_loading = *loading.read();

    rsx! {
        div {
            style: if is_loading { "display:block" }
                   else { "display:none" },
            Spinner {}
        }
        div {
            style: if is_loading { "display:none" }
                   else { "display:block" },
            {children}
        }
    }
}
