// ** page_nav.rs **
// ==> Page navigation arrows + dropdown for the top bar
//     Dynamic: Home + user's subscribed apps

use crate::domain::app_registry::find_app;
use crate::state::user_apps::use_user_apps;
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

stylance::import_crate_style!(
    style,
    "src/components/top_bar/page_nav.module.css"
);

/// (path, label) pair for a navigable page
type PageEntry = (&'static str, &'static str);

/// Build pages list: Home + user's apps
fn use_pages() -> Signal<Vec<PageEntry>> {
    let state = use_user_apps();
    Signal::derive(move || {
        let mut pages: Vec<PageEntry> =
            vec![("/home", "Home")];
        state.apps.with(|apps| {
            let extra = apps.iter().filter_map(|a| {
                find_app(&a.name)
                    .map(|i| (i.path, i.label))
            });
            pages.extend(extra);
        });
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

/// Create a Callback that navigates to a given path
fn use_nav_callback() -> Callback<String> {
    let navigate = use_navigate();
    Callback::new(move |path: String| {
        navigate(&path, Default::default());
    })
}

/// Top bar page navigator: < PageName > with dropdown
#[component]
pub fn PageNav() -> impl IntoView {
    let location = use_location();
    let pages = use_pages();
    let open = RwSignal::new(false);
    let nav = use_nav_callback();

    // Current page index (reactive)
    let idx = Signal::derive(move || {
        let path = location.pathname.get();
        pages.with(|p| current_index(&path, p))
    });

    // Current page label
    let label = Signal::derive(move || {
        pages.with(|p| p[idx.get()].1)
    });

    // Navigate to previous page
    let go_prev = move |_| {
        pages.with(|p| {
            let i = idx.get();
            let prev = if i == 0 {
                p.len() - 1
            } else {
                i - 1
            };
            nav.run(p[prev].0.to_string());
        });
    };

    // Navigate to next page
    let go_next = move |_| {
        pages.with(|p| {
            let i = idx.get();
            let next = (i + 1) % p.len();
            nav.run(p[next].0.to_string());
        });
    };

    view! {
        <div class=style::page_nav>
            <button
                class=style::arrow
                on:click=go_prev
            >
                "\u{2039}"
            </button>
            <button
                class=style::label
                on:click=move |_| {
                    open.update(|v| *v = !*v)
                }
            >
                {label}
            </button>
            <button
                class=style::arrow
                on:click=go_next
            >
                "\u{203A}"
            </button>

            // Dropdown menu
            <Show when=move || open.get()>
                <div class=style::dropdown>
                    {move || pages.get().into_iter()
                        .enumerate()
                        .map(|(i, (path, name))| {
                            let active = move || {
                                idx.get() == i
                            };
                            let p = path.to_string();
                            view! {
                                <button
                                    class=move || {
                                        if active() {
                                            format!(
                                                "{} {}",
                                                style::dropdown_item,
                                                style::active,
                                            )
                                        } else {
                                            style::dropdown_item
                                                .to_string()
                                        }
                                    }
                                    on:click={
                                        let p = p.clone();
                                        move |_| {
                                            open.set(false);
                                            nav.run(
                                                p.clone(),
                                            );
                                        }
                                    }
                                >
                                    {name}
                                </button>
                            }
                        })
                        .collect_view()
                    }
                </div>
            </Show>
        </div>
    }
}
