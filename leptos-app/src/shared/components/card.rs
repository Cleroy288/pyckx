//! GlassCard Component - Reusable glass-morphism card wrapper

use leptos::prelude::*;
use leptos::ev;

/// A generic glass-morphism card that can wrap any content
#[component]
pub fn GlassCard<F>(
    #[prop(optional)] class: &'static str,
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(ev::MouseEvent) + 'static,
{
    let combined_class = if class.is_empty() {
        "glass-card".to_string()
    } else {
        format!("glass-card {}", class)
    };

    view! {
        <button class=combined_class on:click=on_click>
            {children()}
        </button>
    }
}
