//! TopBar — fixed top navigation bar (HomeTopBar)

use super::page_nav::{
    current_pathname, NAV_PAGES,
};
use crate::components::logo::NavLogo;
use crate::components::ui::icon::Icon;
use crate::domain::palette_data::PALETTES;
use crate::state::hooks::use_logout;
use crate::state::palette_provider::use_palette;
use dioxus::prelude::*;

/// Dark mode toggle (neo-brutalist ghost btn)
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
            class: "nav-btn-ghost",
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

/// Theme picker dropdown button
#[component]
fn ThemeBtn() -> Element {
    let mut open = use_signal(|| false);
    let mut palette = use_palette();
    let current_id = palette.palette_id();

    rsx! {
        div { class: "theme-dropdown",
            button {
                class: "nav-btn-ghost",
                onclick: move |_| {
                    open.set(!open());
                },
                "aria-label": "Choose theme",
                // Palette icon (small circle)
                div { class: "theme-btn-dot" }
            }
            if open() {
                div { class: "theme-menu",
                    for p in PALETTES.iter() {
                        button {
                            class: if current_id == p.id {
                                "theme-menu-item active"
                            } else {
                                "theme-menu-item"
                            },
                            onclick: {
                                let id = p.id;
                                move |_| {
                                    palette.set_palette(id);
                                    open.set(false);
                                }
                            },
                            span {
                                class: "theme-menu-dot",
                                style: "background:{p.preview_hex}",
                            }
                            span { "{p.name}" }
                        }
                    }
                }
            }
        }
    }
}

/// TopBar for authenticated pages (neo-brutalist)
#[component]
pub fn HomeTopBar() -> Element {
    let on_logout = use_logout();
    let path = current_pathname();

    rsx! {
        nav { class: "home-nav",
            Link {
                to: "/home",
                class: "logo-wrap no-underline",
                NavLogo {}
            }
            // Center: pill nav links
            div { class: "home-nav-links",
                for (p, label) in NAV_PAGES {
                    {pill_link(p, label, &path)}
                }
            }
            // Right: theme + dark mode + logout
            div { class: "nav-actions",
                ThemeBtn {}
                DarkModeBtn {}
                button {
                    class: "nav-btn",
                    onclick: on_logout,
                    "Logout"
                }
            }
        }
    }
}

/// Single pill nav link with active state
fn pill_link(
    href: &'static str,
    label: &'static str,
    path: &str,
) -> Element {
    let active = if path.starts_with(href) {
        " active"
    } else {
        ""
    };

    rsx! {
        Link {
            to: href,
            class: "{active}",
            "{label}"
        }
    }
}
