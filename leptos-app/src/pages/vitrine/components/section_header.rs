// ** section_header.rs **
// ==> Section header component

use leptos::prelude::*;

stylance::import_crate_style!(style, "src/pages/vitrine/components/section_header.module.css");

#[component]
pub fn SectionHeader(
    #[prop(into)] title: String,
    #[prop(optional)] subtitle: Option<String>,
) -> impl IntoView {
    view! {
        <div class=style::section_header>
            <h2 class=style::section_title>{title}</h2>
            {subtitle.map(|s| view! {
                <p class=style::section_subtitle>{s}</p>
            })}
        </div>
    }
}
