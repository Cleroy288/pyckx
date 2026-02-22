//! App Grid Component - Grid container for app cards

use leptos::prelude::*;

stylance::import_crate_style!(style, "src/components/dashboard/app_grid.module.css");

#[component]
pub fn AppGrid(
    #[prop(optional, into)] title: Option<String>,
    children: Children,
) -> impl IntoView {
    let section_title = title.unwrap_or_else(|| "Your Apps".to_string());

    view! {
        <section class=style::apps_section>
            <h2 class=style::section_title>{section_title}</h2>
            <div class=style::apps_grid>
                {children()}
            </div>
        </section>
    }
}
