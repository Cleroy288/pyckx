//! TopBar - Transparent top navigation bar components
//! 
//! Provides two TopBar variants:
//! - VitrineTopBar: For public pages (vitrine, login) with full navigation
//! - HomeTopBar: For authenticated home page with minimal navigation

use crate::api;
use crate::state::use_auth;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

// Import the top_bar CSS module from the same folder
stylance::import_crate_style!(top_bar_style, "src/components/top_bar/top_bar.module.css");

// ** VitrineTopBar **
// ==> TopBar for public pages: Logo + Services + Pricing + Login/Logout
// @returns: Full navigation bar for vitrine and login pages
#[component]
pub fn VitrineTopBar() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let navigate_to = RwSignal::new(Option::<String>::None);

    // Effect to handle navigation (must be in sync context)
    Effect::new(move |_| {
        if let Some(path) = navigate_to.get() {
            navigate(&path, Default::default());
            navigate_to.set(None);
        }
    });

    // Handle logout
    let on_logout = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        auth.is_loading.set(true);

        spawn_local(async move {
            match api::logout().await {
                Ok(()) => {
                    auth.clear_user();
                    auth.is_loading.set(false);
                    navigate_to.set(Some("/".to_string()));
                }
                Err(e) => {
                    web_sys::console::warn_1(&format!("Logout failed: {}", e).into());
                    auth.set_error(format!("Logout failed: {}", e));
                    auth.clear_user();
                    auth.is_loading.set(false);
                }
            }
        });
    };

    view! {
        <nav class=top_bar_style::top_bar>
            <div class=top_bar_style::nav_content>
                <A href="/" attr:class=top_bar_style::nav_logo>"PYCKX"</A>
                <div class=top_bar_style::nav_links>
                    <a href="#services" class=top_bar_style::nav_link>"Services"</a>
                    <a href="#pricing" class=top_bar_style::nav_link>"Pricing"</a>

                    // Show Login or Logout based on auth state
                    <Show
                        when=move || auth.user.get().is_some()
                        fallback=move || view! {
                            <A href="/login" attr:class=top_bar_style::nav_link_primary>"Login"</A>
                        }
                    >
                        <A href="/home" attr:class=top_bar_style::nav_link>
                            {move || auth.user.get().map(|u| u.username).unwrap_or_default()}
                        </A>
                        <a
                            href="#"
                            class=top_bar_style::nav_link_primary
                            on:click=on_logout
                        >
                            "Logout"
                        </a>
                    </Show>
                </div>
            </div>
        </nav>
    }
}

// ** HomeTopBar **
// ==> Simplified TopBar for authenticated home page: Logo + Profile + Logout
// @returns: Minimal navigation bar for home page
#[component]
pub fn HomeTopBar() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let navigate_to = RwSignal::new(Option::<String>::None);

    // Effect to handle navigation (must be in sync context)
    Effect::new(move |_| {
        if let Some(path) = navigate_to.get() {
            navigate(&path, Default::default());
            navigate_to.set(None);
        }
    });

    // Handle logout
    let on_logout = move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        auth.is_loading.set(true);

        spawn_local(async move {
            match api::logout().await {
                Ok(()) => {
                    auth.clear_user();
                    auth.is_loading.set(false);
                    navigate_to.set(Some("/".to_string()));
                }
                Err(e) => {
                    web_sys::console::warn_1(&format!("Logout failed: {}", e).into());
                    auth.set_error(format!("Logout failed: {}", e));
                    auth.clear_user();
                    auth.is_loading.set(false);
                }
            }
        });
    };

    view! {
        <nav class=top_bar_style::top_bar>
            <div class=top_bar_style::nav_content>
                <A href="/" attr:class=top_bar_style::nav_logo>"PYCKX"</A>
                <div class=top_bar_style::nav_links>
                    <A href="/profile" attr:class=top_bar_style::nav_link>"Profile"</A>
                    <a
                        href="#"
                        class=top_bar_style::nav_link_primary
                        on:click=on_logout
                    >
                        "Logout"
                    </a>
                </div>
            </div>
        </nav>
    }
}
