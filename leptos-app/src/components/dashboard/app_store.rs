// ** app_store.rs **
// ==> Catalog of all available apps with add/remove

use crate::components::ui::button::{
    btn_class, ButtonSize, ButtonVariant,
};
use crate::components::ui::card::CardGrid;
use crate::components::ui::icon::Icon;
use crate::domain::app_registry::{AppInfo, APP_REGISTRY};
use crate::state::user_apps::UserAppsState;
use crate::state::user_apps::use_user_apps;
use leptos::prelude::*;

stylance::import_crate_style!(
    style,
    "src/components/dashboard/app_store.module.css"
);

/// Grid of all platform apps with add/remove toggle
#[component]
pub fn AppStore() -> impl IntoView {
    let state = use_user_apps();

    view! {
        <CardGrid>
            {APP_REGISTRY.iter().map(|info| {
                app_card(info, state)
            }).collect_view()}
        </CardGrid>
    }
}

/// Single app card with toggle button
fn app_card(
    info: &'static AppInfo,
    state: UserAppsState,
) -> impl IntoView {
    // Reactive: is this app in user's list?
    let added = Signal::derive(move || {
        state.apps.with(|v| {
            v.iter().any(|a| a.name == info.name)
        })
    });

    // Toggle add/remove
    let toggle = move |_| {
        if added.get_untracked() {
            state.remove_app(info.name);
        } else {
            state.add_app(info.name);
        }
    };

    // Reactive button class + label
    let btn_cls = move || {
        let v = if added.get() {
            ButtonVariant::Outline
        } else {
            ButtonVariant::Primary
        };
        btn_class(v, ButtonSize::Small)
    };
    let label = move || {
        if added.get() { "Remove" } else { "Add" }
    };

    view! {
        <div class=style::card>
            <div class=style::header>
                <div class=style::icon>
                    <Icon icon_name=info.icon />
                </div>
                <h3 class=style::name>{info.label}</h3>
            </div>
            <p class=style::desc>{info.description}</p>
            <button class=btn_cls on:click=toggle>
                {label}
            </button>
        </div>
    }
}
