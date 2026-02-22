// ** nav_card.rs **
// ==> Clickable navigation card with icon, title,
//     and description. Used on home and intello pages.

use crate::components::ui::icon::Icon;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/card/nav_card.module.css"
);

/// Clickable navigation tile with icon + title + desc
#[component]
pub fn NavCard(
    /// Card title
    #[prop(into)]
    title: String,
    /// Short description below the title
    #[prop(into)]
    description: String,
    /// Icon name (from Icon component)
    #[prop(into)]
    icon: String,
    /// Click handler for navigation
    on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            class=style::card
            on:click=move |_| on_click.run(())
        >
            <div class=style::icon>
                <Icon icon_name=icon />
            </div>
            <span class=style::title>{title}</span>
            <span class=style::desc>
                {description}
            </span>
        </button>
    }
}
