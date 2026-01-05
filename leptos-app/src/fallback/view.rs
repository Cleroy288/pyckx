//! Fallback View Module

use leptos::prelude::*;
use crate::shared::components::{Icon, IconType};

#[component]
pub fn FallbackView(current_view: String, on_back: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="fallback-view">
            <div class="fallback-card">
                <div class="fallback-icon">
                    <Icon icon=IconType::Brain />
                </div>
                <h2>{current_view}</h2>
                <p>"This feature is coming soon. Stay tuned for updates!"</p>
                <button class="back-button" on:click=move |_| on_back.set("home".to_string())>
                    <Icon icon=IconType::ArrowLeft />
                    "Back to Home"
                </button>
            </div>
        </div>
    }
}
