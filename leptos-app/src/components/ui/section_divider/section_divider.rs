// ** section_divider.rs **
// ==> Centered label with horizontal lines on each side

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/section_divider/\
     section_divider.module.css"
);

/// Horizontal divider with a centered text label
/// ———— LABEL ————
#[component]
pub fn SectionDivider(
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <div class=style::divider>
            <span class=style::line></span>
            {label}
            <span class=style::line></span>
        </div>
    }
}
