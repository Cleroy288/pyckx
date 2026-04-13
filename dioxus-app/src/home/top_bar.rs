//! Fixed top navigation bar shown on authenticated pages.
//!
//! Left: clickable Pyckx logo. Center: pill nav links.
//! Right: theme picker, dark-mode toggle, logout button.

use crate::home::logo::NavLogo;
use crate::state::hooks::use_logout;
use crate::state::palette_provider::use_palette;
use crate::theme::PALETTES;
use crate::ui::Icon;
use dioxus::prelude::*;

/// Default route used when the browser path is unavailable.
const DEFAULT_PATH: &str = "/home";

/// Navigation pills rendered in the top bar.
const NAV_PAGES: [(&str, &str); 2] = [
    ("/home", "Accueil"),
    ("/courses", "Cours"),
];

/// Top bar for authenticated pages.
#[component]
pub fn HomeTopBar() -> Element {
    let on_logout = use_logout();
    let path = read_pathname();

    rsx! {
        nav { class: "home-nav",
            Link {
                to: DEFAULT_PATH,
                class: "logo-wrap no-underline",
                NavLogo {}
            }
            div { class: "home-nav-links",
                for (href, label) in NAV_PAGES {
                    {pill_link(href, label, &path)}
                }
            }
            div { class: "nav-actions",
                ThemeControls {}
                button {
                    class: "nav-btn",
                    onclick: on_logout,
                    "Logout"
                }
            }
        }
    }
}

/// Palette dropdown + dark-mode toggle group.
#[component]
fn ThemeControls() -> Element {
    let mut palette = use_palette();
    let mut open = use_signal(|| false);
    let current_id = palette.palette_id();
    let dark_label = if palette.is_dark() { "Sun" } else { "Moon" };

    rsx! {
        div { class: "theme-dropdown",
            button {
                class: "nav-btn-ghost",
                onclick: move |_| open.set(!open()),
                "aria-label": "Choose theme",
                div { class: "theme-btn-dot" }
            }
            if open() {
                div { class: "theme-menu",
                    for p in PALETTES.iter() {
                        button {
                            class: menu_item_class(current_id == p.id),
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
        button {
            class: "nav-btn-ghost",
            onclick: move |_| palette.toggle_dark(),
            "aria-label": "Toggle dark mode",
            Icon { icon_name: dark_label.to_string() }
        }
    }
}

/// Render a single active-aware navigation pill.
fn pill_link(
    href: &'static str,
    label: &'static str,
    path: &str,
) -> Element {
    let extra = if is_active_path(path, href) {
        " active"
    } else {
        ""
    };
    rsx! {
        Link { to: href, class: "{extra}", "{label}" }
    }
}

/// Whether `path` matches (or sits under) `target`.
fn is_active_path(path: &str, target: &str) -> bool {
    path.starts_with(target)
}

/// CSS class for a theme-menu item, depending on selection.
fn menu_item_class(selected: bool) -> &'static str {
    if selected {
        "theme-menu-item active"
    } else {
        "theme-menu-item"
    }
}

/// Read the current pathname from the browser `Location`.
fn read_pathname() -> String {
    web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| DEFAULT_PATH.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_active_path_exact_match_returns_true() {
        assert!(is_active_path("/home", "/home"));
    }

    #[test]
    fn test_is_active_path_subpath_returns_true() {
        assert!(is_active_path("/courses/42", "/courses"));
    }

    #[test]
    fn test_is_active_path_different_route_returns_false() {
        assert!(!is_active_path("/home", "/courses"));
    }

    #[test]
    fn test_menu_item_class_selected_has_active() {
        assert_eq!(menu_item_class(true), "theme-menu-item active");
    }

    #[test]
    fn test_menu_item_class_unselected_is_plain() {
        assert_eq!(menu_item_class(false), "theme-menu-item");
    }
}
