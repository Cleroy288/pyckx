//! TopBar - Transparent top navigation bar
//!
//! Two variants:
//! - VitrineTopBar: public pages (logo + nav + login)
//! - HomeTopBar: authenticated pages (logo + nav + logout)

use super::page_nav::PageNav;
use crate::components::ui::icon::Icon;
use crate::state::hooks::use_logout;
use crate::state::palette_provider::use_palette;
use crate::state::use_auth;
use leptos::prelude::*;
use leptos_router::components::A;

stylance::import_crate_style!(
    style,
    "src/components/top_bar/top_bar.module.css"
);

/// TopBar for public pages
#[component]
pub fn VitrineTopBar() -> impl IntoView {
    let auth = use_auth();
    let on_logout = use_logout();

    view! {
        <nav class=style::top_bar>
            <div class=style::nav_content>
                <A
                    href=move || {
                        if auth.user.get().is_some() {
                            "/home".to_string()
                        } else {
                            "/".to_string()
                        }
                    }
                    attr:class=style::nav_logo
                >
                    <img
                        src="/favicon.svg"
                        alt="Pyckx"
                        class=style::nav_logo_icon
                    />
                    "PYCKX"
                </A>
                <div class=style::nav_links>
                    <a
                        href="#services"
                        class=style::nav_link
                    >"Services"</a>
                    <a
                        href="#pricing"
                        class=style::nav_link
                    >"Pricing"</a>
                    <Show
                        when=move || {
                            auth.user.get().is_some()
                        }
                        fallback=move || view! {
                            <A
                                href="/login"
                                attr:class=style::nav_link_primary
                            >"Login"</A>
                        }
                    >
                        <A
                            href="/home"
                            attr:class=style::nav_link
                        >
                            {move || {
                                auth.user
                                    .get()
                                    .map(|u| u.username)
                                    .unwrap_or_default()
                            }}
                        </A>
                        <a
                            href="#"
                            class=style::nav_link_primary
                            on:click=on_logout.clone()
                        >"Logout"</a>
                    </Show>
                </div>
            </div>
        </nav>
    }
}

/// Dark mode toggle button for the top bar
#[component]
fn DarkModeBtn() -> impl IntoView {
    let palette = use_palette();
    let icon_name = move || {
        if palette.is_dark() { "Sun" } else { "Moon" }
    };

    view! {
        <button
            class=style::nav_icon_btn
            on:click=move |_| palette.toggle_dark()
            aria-label="Toggle dark mode"
        >
            <Icon icon_name=icon_name() />
        </button>
    }
}

/// TopBar for authenticated pages
#[component]
pub fn HomeTopBar() -> impl IntoView {
    let on_logout = use_logout();

    view! {
        <nav class=style::top_bar>
            <div class=style::nav_content>
                <A
                    href="/home"
                    attr:class=style::nav_logo
                >
                    <img
                        src="/favicon.svg"
                        alt="Pyckx"
                        class=style::nav_logo_icon
                    />
                    "PYCKX"
                </A>
                <PageNav />
                <div class=style::nav_links>
                    <DarkModeBtn />
                    <a
                        href="#"
                        class=style::nav_link_primary
                        on:click=on_logout
                    >"Logout"</a>
                </div>
            </div>
        </nav>
    }
}
