//! IntelloHeader Component - Page header with icon and title

use leptos::prelude::*;
use crate::shared::components::{Icon, IconType};

#[component]
pub fn IntelloHeader() -> impl IntoView {
    view! {
        <header class="intello-header">
            <div class="intello-header__icon">
                <Icon icon=IconType::Brain />
            </div>
            <h1 class="intello-header__title">"Intello"</h1>
            <p class="intello-header__subtitle">"Create AI-powered quizzes and master your knowledge with ease."</p>
        </header>
    }
}
