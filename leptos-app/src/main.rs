//! Leptos App - Entry Point & Router
//!
//! A modular, DRY architecture for the Leptos WASM app.

mod shared;
mod home;
mod intello;
mod fallback;

use leptos::prelude::*;
use web_sys;
use shared::components::{Icon, IconType};
use home::HomePage;
use intello::IntelloPage;
use fallback::FallbackView;

// ============================================================
// HELPERS
// ============================================================

/// Extract the initial view from the browser's URL path
fn get_initial_view() -> String {
    let window = web_sys::window().expect("no window");
    let pathname = window.location().pathname().unwrap_or_default();
    
    // Extract view from path like "/mermaid" -> "mermaid"
    let view = pathname.trim_start_matches('/').to_string();
    let view = view.split('/').next().unwrap_or("").to_string();
    
    // Default to "home" if empty or root
    if view.is_empty() {
        "home".to_string()
    } else {
        view
    }
}

// ============================================================
// MAIN APP - Router
// ============================================================

#[component]
fn App() -> impl IntoView {
    let (current_view, set_current_view) = signal(get_initial_view());

    view! {
        <div class="app-container">
            <div class="main-content">
                {move || {
                    let view = current_view.get();
                    match view.as_str() {
                        "home" => view! { <HomePage on_navigate=set_current_view /> }.into_any(),
                        "intello" => view! { <IntelloPage on_navigate=set_current_view /> }.into_any(),
                        _ => view! { <FallbackView current_view=view on_back=set_current_view /> }.into_any(),
                    }
                }}
            </div>
        </div>
    }
}

// ============================================================
// ENTRY POINT
// ============================================================

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
