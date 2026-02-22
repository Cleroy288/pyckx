// ** page_layout.rs **
// ==> Shared page container with side lines

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/page_layout/page_layout.module.css"
);

/// Full-height page wrapper with side lines
/// and centered 1200px content column.
#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <main class=style::page>
            <div class=style::content>
                {children()}
            </div>
        </main>
    }
}
