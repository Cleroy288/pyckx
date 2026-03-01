// ** welcome_header.rs **
// ==> Welcome greeting inside HeroBanner

use crate::components::ui::hero_banner::HeroBanner;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/dashboard/welcome_header.module.css"
);

/// Greeting banner with username
#[component]
pub fn WelcomeHeader(
    #[prop(into)] username: Signal<String>,
    #[prop(optional, into)] subtitle: Option<String>,
) -> impl IntoView {
    let sub = subtitle.unwrap_or_else(|| {
        "Select an app to get started".to_string()
    });

    view! {
        <HeroBanner>
            <h1 class=style::greeting>
                "Welcome, "
                <span class=style::username>
                    {move || username.get()}
                </span>
            </h1>
            <p class=style::subtitle>{sub}</p>
        </HeroBanner>
    }
}
