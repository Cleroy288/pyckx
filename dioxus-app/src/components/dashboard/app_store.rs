//! AppStore — catalog of all apps with add/remove

use crate::components::ui::card::CardGrid;
use crate::components::ui::icon::Icon;
use crate::domain::app_registry::{AppInfo, APP_REGISTRY};
use crate::state::user_apps::use_user_apps;
use crate::state::user_apps::UserAppsState;
use dioxus::prelude::*;

/// Grid of all platform apps with add/remove toggle
#[component]
pub fn AppStore() -> Element {
    let state = use_user_apps();

    rsx! {
        CardGrid {
            for info in APP_REGISTRY.iter() {
                AppStoreCard {
                    info: info,
                    state: state,
                }
            }
        }
    }
}

/// Single app card with toggle button
#[component]
fn AppStoreCard(
    info: &'static AppInfo,
    state: UserAppsState,
) -> Element {
    // Reactive: is this app in user's list?
    let added = use_memo(move || {
        state.apps.read().iter().any(|a| {
            a.name == info.name
        })
    });

    // Toggle add/remove
    let toggle = move |_| {
        if *added.peek() {
            state.remove_app(info.name);
        } else {
            state.add_app(info.name);
        }
    };

    // Button styling and label
    let btn_cls = if *added.read() {
        "px-3 py-1.5 text-sm font-medium \
         border border-[var(--color-border)] \
         bg-transparent \
         text-[var(--color-text-primary)] \
         rounded-none cursor-pointer \
         transition-colors duration-200 \
         hover:border-[var(--color-primary)] \
         hover:text-[var(--color-primary)]"
    } else {
        "px-3 py-1.5 text-sm font-medium \
         border border-transparent \
         bg-[var(--color-primary)] \
         text-[var(--color-background)] \
         rounded-none cursor-pointer \
         transition-colors duration-200 \
         hover:brightness-110"
    };
    let label =
        if *added.read() { "Remove" } else { "Add" };

    rsx! {
        div {
            class: "flex flex-col gap-3 p-6 \
                bg-[var(--glass-bg)] \
                backdrop-blur-[20px] \
                border border-[var(--color-border)] \
                transition-[border-color] duration-200 \
                hover:border-[var(--color-primary)]",
            div {
                class: "flex items-center gap-3",
                div {
                    class: "w-10 h-10 shrink-0 \
                        flex items-center justify-center \
                        bg-[color-mix(in_srgb,var(--primary)_10%,transparent)] \
                        text-[var(--color-primary)] \
                        [&_svg]:w-[22px] [&_svg]:h-[22px]",
                    Icon {
                        icon_name: info.icon.to_string(),
                    }
                }
                h3 {
                    class: "m-0 text-base font-semibold \
                        text-[var(--color-text-primary)]",
                    "{info.label}"
                }
            }
            p {
                class: "text-sm \
                    text-[var(--color-text-secondary)] \
                    m-0 leading-relaxed flex-1",
                "{info.description}"
            }
            button {
                class: "{btn_cls}",
                onclick: toggle,
                "{label}"
            }
        }
    }
}
