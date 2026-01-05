//! Home Page View

use leptos::prelude::*;
use crate::home::components::Hero;

#[component]
pub fn HomePage(on_navigate: WriteSignal<String>) -> impl IntoView {
    view! {
        <div class="home-page">
            <Hero on_start=move |_| on_navigate.set("intello".to_string()) />
        </div>
    }
}
