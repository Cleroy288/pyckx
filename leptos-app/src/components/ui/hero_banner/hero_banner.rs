// ** hero_banner.rs **
// ==> Shared hero banner with glass + grid overlay

use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/ui/hero_banner/hero_banner.module.css"
);

/// Glass hero banner with grid overlay and
/// optional gradient title.
#[component]
pub fn HeroBanner(
    /// Gradient title text. Omit for custom
    /// title via children.
    #[prop(optional, into)]
    title: Option<String>,
    /// Body content (subtitle, buttons, etc.)
    children: Children,
) -> impl IntoView {
    view! {
        <header class=style::banner>
            {title.map(|t| view! {
                <h1 class=style::banner_title>{t}</h1>
            })}
            <div class=style::banner_body>
                {children()}
            </div>
        </header>
    }
}
