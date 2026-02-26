//! PageNav — page navigation arrows + dropdown
//! Dynamic: Home + user's subscribed apps

use crate::domain::app_registry::find_app;
use crate::state::user_apps::use_user_apps;
use dioxus::prelude::*;
use dioxus_router::Navigator;

/// (path, label) pair for a navigable page
type PageEntry = (&'static str, &'static str);

/// Build pages list: Home + user's apps
fn use_pages() -> Memo<Vec<PageEntry>> {
    let state = use_user_apps();
    use_memo(move || {
        let mut pages: Vec<PageEntry> =
            vec![("/home", "Home")];
        let apps_guard = state.apps.read();
        let extra = apps_guard.iter()
            .filter_map(|a| {
                find_app(&a.name)
                    .map(|i| (i.path, i.label))
            });
        pages.extend(extra);
        pages
    })
}

/// Find index of current page in pages list
fn current_index(
    path: &str,
    pages: &[PageEntry],
) -> usize {
    pages
        .iter()
        .position(|(p, _)| path.starts_with(p))
        .unwrap_or(0)
}

/// Read browser pathname via web_sys
fn current_pathname() -> String {
    web_sys::window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_else(|| "/home".to_string())
}

/// CSS class for the nav container
const PAGE_NAV: &str = "page-nav-lines \
    absolute left-1/2 top-1/2 \
    -translate-x-1/2 -translate-y-1/2 z-0 \
    flex items-center \
    bg-[var(--color-background)] \
    border border-[var(--color-border)]";

const ARROW: &str = "flex items-center \
    justify-center \
    w-8 py-1 bg-transparent border-0 \
    cursor-pointer text-base font-semibold \
    text-[var(--color-text-secondary)] \
    transition-colors duration-200 \
    hover:text-[var(--color-primary)]";

const LABEL: &str = "flex items-center \
    justify-center \
    w-28 py-1 border-0 \
    border-x border-[var(--color-border)] \
    bg-transparent cursor-pointer \
    text-xs font-semibold \
    text-[var(--color-text-primary)] \
    uppercase tracking-wider whitespace-nowrap \
    transition-colors duration-200 \
    hover:text-[var(--color-primary)]";

const DROPDOWN: &str = "absolute top-full \
    left-1/2 -translate-x-1/2 z-10 \
    flex flex-col min-w-full \
    bg-[var(--color-background)] \
    border border-[var(--color-border)] \
    border-t-0";

const DROPDOWN_ITEM: &str = "flex items-center \
    justify-center px-4 py-2 \
    bg-transparent border-0 \
    border-t border-[var(--color-border)] \
    cursor-pointer text-xs font-semibold \
    text-[var(--color-text-primary)] \
    uppercase tracking-wider whitespace-nowrap \
    transition-colors duration-200 \
    hover:text-[var(--color-primary)]";

/// Top bar page navigator: < PageName > dropdown
#[component]
pub fn PageNav() -> Element {
    let pages = use_pages();
    let open = use_signal(|| false);
    let nav = use_navigator();
    let path = current_pathname();

    let idx =
        current_index(&path, &pages());
    let label = pages()[idx].1;

    rsx! {
        div { class: "{PAGE_NAV}",
            button {
                class: "{ARROW}",
                onclick: {
                    let nav = nav.clone();
                    let pages = pages.clone();
                    move |_| {
                        let p = pages();
                        let prev = if idx == 0 {
                            p.len() - 1
                        } else {
                            idx - 1
                        };
                        nav.push(p[prev].0);
                    }
                },
                "\u{2039}"
            }
            button {
                class: "{LABEL}",
                onclick: move |_| {
                    let val = (open)();
                    let mut o = open;
                    o.set(!val);
                },
                "{label}"
            }
            button {
                class: "{ARROW}",
                onclick: {
                    let nav = nav.clone();
                    let pages = pages.clone();
                    move |_| {
                        let p = pages();
                        let next = (idx + 1) % p.len();
                        nav.push(p[next].0);
                    }
                },
                "\u{203A}"
            }
            // Dropdown menu
            if (open)() {
                div { class: "{DROPDOWN}",
                    for (i, (p, name)) in pages()
                        .into_iter().enumerate()
                    {
                        {dropdown_item(
                            i, idx, p, name,
                            nav.clone(),
                            open,
                        )}
                    }
                }
            }
        }
    }
}

/// Single dropdown item
fn dropdown_item(
    i: usize,
    active_idx: usize,
    path: &'static str,
    name: &'static str,
    nav: Navigator,
    open: Signal<bool>,
) -> Element {
    let active_cls = if i == active_idx {
        " text-[var(--color-primary)]"
    } else {
        ""
    };

    rsx! {
        button {
            class: "{DROPDOWN_ITEM}{active_cls}",
            onclick: move |_| {
                let mut o = open;
                o.set(false);
                nav.push(path);
            },
            "{name}"
        }
    }
}
