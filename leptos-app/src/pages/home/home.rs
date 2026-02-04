//! Home Page Component - Protected page showing user greeting

use crate::components::top_bar::HomeTopBar;
use crate::components::ui::card::{Card, CardVariant};
use crate::state::use_auth;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

stylance::import_crate_style!(home_style, "src/pages/home/home.module.css");

#[component]
pub fn HomePage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    // Effect to handle navigation (must be in sync context)
    // Only redirect to login if:
    // 1. We're done checking the session (is_checking_session is false)
    // 2. AND the user is not authenticated
    Effect::new(move |_| {
        let is_checking = auth.is_checking_session.get();
        let has_user = auth.user.get().is_some();

        // Only redirect if we're done checking and there's no user
        if !is_checking && !has_user {
            navigate("/login", Default::default());
        }
    });

    view! {
        <>
            <HomeTopBar />
            <div class="page-container">
                <Card variant=CardVariant::Solid class=home_style::home_card.to_string()>
                    // Show loading while checking session
                    <Show
                        when=move || !auth.is_checking_session.get()
                        fallback=|| view! { <p>"Checking session..."</p> }
                    >
                        // Show content only when authenticated
                        <Show
                            when=move || auth.user.get().is_some()
                            fallback=|| view! { <p>"Redirecting to login..."</p> }
                        >
                            <h1 class=home_style::greeting>
                                "Hello: "
                                <span class=home_style::username>
                                    {move || auth.user.get().map(|u| u.username).unwrap_or_default()}
                                </span>
                            </h1>

                            <div class=home_style::user_info>
                                <p>
                                    <strong>"Email: "</strong>
                                    {move || auth.user.get().map(|u| u.email).unwrap_or_default()}
                                </p>
                                <p>
                                    <strong>"Role: "</strong>
                                    {move || auth.user.get().map(|u| u.role).unwrap_or_default()}
                                </p>
                            </div>
                        </Show>
                    </Show>
                </Card>
            </div>
        </>
    }
}

