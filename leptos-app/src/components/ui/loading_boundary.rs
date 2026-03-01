// ** loading_boundary.rs **
// ==> Show spinner while loading, children when ready

use crate::components::ui::spinner::Spinner;
use leptos::prelude::*;

/// Wrapper: shows Spinner while loading,
/// renders children when done.
/// Uses style:display toggle to avoid
/// Show + Children ownership issues.
#[component]
pub fn LoadingBoundary(
    /// True while data is loading
    loading: RwSignal<bool>,
    /// Content to show once loaded
    children: Children,
) -> impl IntoView {
    view! {
        <div style:display=move || {
            if loading.get() { "block" } else { "none" }
        }>
            <Spinner />
        </div>
        <div style:display=move || {
            if loading.get() { "none" } else { "block" }
        }>
            {children()}
        </div>
    }
}
