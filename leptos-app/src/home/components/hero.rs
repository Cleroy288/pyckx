//! Hero Component - Main landing section

use leptos::prelude::*;
use crate::shared::components::{Icon, IconType};

#[component]
pub fn Hero<F>(on_start: F) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + 'static,
{
    view! {
        <div class="home-hero">
            <div class="home-hero__icon">
                <Icon icon=IconType::Brain />
            </div>
            <h1 class="home-hero__title">"Welcome to Pyckx"</h1>
            <p class="home-hero__subtitle">"Your AI-powered learning companion"</p>
            <button class="home-hero__button" on:click=on_start>
                "Get Started"
                <Icon icon=IconType::ArrowRight />
            </button>
        </div>
    }
}
