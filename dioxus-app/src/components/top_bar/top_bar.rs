//! TopBar — fixed top navigation bar
//!
//! Two variants:
//! - VitrineTopBar: public pages (logo + nav + login)
//! - HomeTopBar: authenticated pages (logo + nav + logout)

use super::page_nav::PageNav;
use crate::components::ui::icon::Icon;
use crate::state::hooks::use_logout;
use crate::state::palette_provider::use_palette;
use crate::state::use_auth;
use dioxus::prelude::*;

/// CSS class constants for top bar cells
const TOP_BAR: &str = "top-bar-grid \
    fixed top-0 inset-x-0 z-[100] \
    bg-[var(--glass-bg)] backdrop-blur-[24px] \
    border-b border-[var(--color-border)]";

const NAV_CONTENT: &str = "relative z-10 \
    max-w-[calc(1200px+4rem)] mx-auto px-8 \
    flex items-stretch justify-between";

const NAV_LOGO: &str = "relative z-[1] \
    flex items-center gap-2 px-6 py-4 \
    border-x border-[var(--color-border)] \
    bg-[var(--color-background)] \
    text-2xl font-extrabold \
    text-[var(--color-primary)] no-underline \
    uppercase tracking-wider \
    transition-opacity duration-200 \
    hover:opacity-80";

const NAV_LINK: &str = "relative z-[1] \
    flex items-center px-6 py-4 \
    border-l border-[var(--color-border)] \
    bg-[var(--color-background)] \
    text-sm font-medium \
    text-[var(--color-text-primary)] no-underline \
    whitespace-nowrap \
    transition-colors duration-200 \
    hover:text-[var(--color-primary)]";

const NAV_PRIMARY: &str = "relative z-[1] \
    flex items-center px-6 py-4 \
    border-l border-[var(--color-border)] \
    text-sm font-semibold \
    text-[var(--color-background)] no-underline \
    whitespace-nowrap \
    bg-[var(--color-primary)] \
    hover:brightness-110 \
    transition-all duration-200";

const ICON_BTN: &str = "relative z-[1] \
    flex items-center justify-center \
    w-[3.25rem] p-0 border-0 \
    border-l border-[var(--color-border)] \
    bg-[var(--color-background)] \
    text-[var(--color-text-primary)] \
    cursor-pointer \
    transition-colors duration-200 \
    hover:text-[var(--color-primary)] \
    [&_svg]:w-[1.125rem] [&_svg]:h-[1.125rem]";

/// TopBar for public pages
#[component]
pub fn VitrineTopBar() -> Element {
    let auth = use_auth();
    let on_logout = use_logout();
    let has_user = auth.user.read().is_some();

    let home_href = if has_user {
        "/home"
    } else {
        "/"
    };

    let username = auth
        .user
        .read()
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();

    rsx! {
        nav { class: "{TOP_BAR}",
            div { class: "{NAV_CONTENT}",
                Link {
                    to: "{home_href}",
                    class: "{NAV_LOGO}",
                    img {
                        src: "/assets/favicon.svg",
                        alt: "Pyckx",
                        class: "w-14 h-14 shrink-0 \
                            -my-3.5",
                    }
                    "PYCKX"
                }
                div {
                    class: "flex items-stretch",
                    a {
                        href: "#services",
                        class: "{NAV_LINK}",
                        "Services"
                    }
                    a {
                        href: "#pricing",
                        class: "{NAV_LINK}",
                        "Pricing"
                    }
                    if has_user {
                        Link {
                            to: "/home",
                            class: "{NAV_LINK}",
                            "{username}"
                        }
                        a {
                            href: "#",
                            class: "{NAV_PRIMARY}",
                            onclick: on_logout,
                            "Logout"
                        }
                    } else {
                        Link {
                            to: "/login",
                            class: "{NAV_PRIMARY}",
                            "Login"
                        }
                    }
                }
            }
        }
    }
}

/// Dark mode toggle button for the top bar
#[component]
fn DarkModeBtn() -> Element {
    let mut palette = use_palette();
    let icon_name = if palette.is_dark() {
        "Sun"
    } else {
        "Moon"
    };

    rsx! {
        button {
            class: "{ICON_BTN}",
            onclick: move |_| {
                palette.toggle_dark();
            },
            "aria-label": "Toggle dark mode",
            Icon {
                icon_name: icon_name.to_string(),
            }
        }
    }
}

/// TopBar for authenticated pages
#[component]
pub fn HomeTopBar() -> Element {
    let on_logout = use_logout();

    rsx! {
        nav { class: "{TOP_BAR}",
            div { class: "{NAV_CONTENT}",
                Link {
                    to: "/home",
                    class: "{NAV_LOGO}",
                    img {
                        src: "/assets/favicon.svg",
                        alt: "Pyckx",
                        class: "w-14 h-14 shrink-0 \
                            -my-3.5",
                    }
                    "PYCKX"
                }
                PageNav {}
                div {
                    class: "flex items-stretch",
                    DarkModeBtn {}
                    a {
                        href: "#",
                        class: "{NAV_PRIMARY}",
                        onclick: on_logout,
                        "Logout"
                    }
                }
            }
        }
    }
}
