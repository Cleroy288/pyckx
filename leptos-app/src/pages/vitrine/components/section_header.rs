// ** section_header.rs **
// ==> Section header component

use leptos::prelude::*;

stylance::import_crate_style!(vitrine_style, "src/pages/vitrine/vitrine.module.css");

#[component]
pub fn SectionHeader(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
) -> impl IntoView {
    view! {
        <div class=vitrine_style::section_header>
            <h2 class=vitrine_style::section_title>{title}</h2>
            {subtitle.map(|s| view! {
                <p class=vitrine_style::section_subtitle>{s}</p>
            })}
        </div>
    }
}
