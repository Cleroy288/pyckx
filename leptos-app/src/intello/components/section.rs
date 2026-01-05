//! Section Component - Wrapper for content sections

use leptos::prelude::*;
use crate::shared::components::{Icon, IconType};

#[component]
pub fn Section(
    title: &'static str,
    subtitle: &'static str,
    icon: IconType,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="section-card">
            <div class="section-header">
                <div class="section__icon">
                    <Icon icon=icon />
                </div>
                <div>
                    <h2 class="section__title">{title}</h2>
                    <p class="section__subtitle">{subtitle}</p>
                </div>
            </div>
            <div class="games-grid">
                {children()}
            </div>
        </section>
    }
}
