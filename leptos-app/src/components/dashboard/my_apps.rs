// ** my_apps.rs **
// ==> User's subscribed apps as nav cards

use crate::components::ui::button::nav_callback;
use crate::components::ui::card::{CardGrid, NavCard};
use crate::components::ui::empty_state::EmptyState;
use crate::domain::app_registry::find_app;
use crate::state::user_apps::use_user_apps;
use leptos::prelude::*;

/// Shows user's apps as clickable nav cards
#[component]
pub fn MyApps() -> impl IntoView {
    let state = use_user_apps();

    // Derive list of (label, description, icon, path)
    let entries = Signal::derive(move || {
        state.apps.with(|apps| {
            apps.iter()
                .filter_map(|a| find_app(&a.name))
                .collect::<Vec<_>>()
        })
    });

    // True when user has no apps
    let is_empty =
        Signal::derive(move || entries.with(Vec::is_empty));

    view! {
        <Show
            when=move || !is_empty.get()
            fallback=|| view! {
                <EmptyState
                    icon="Plus"
                    message="No apps yet"
                    description="Add apps from the \
                        catalog below."
                />
            }
        >
            <CardGrid>
                {move || entries.get().into_iter().map(
                    |info| view! {
                        <NavCard
                            title=info.label
                            description=info.description
                            icon=info.icon
                            on_click=nav_callback(
                                info.path,
                            )
                        />
                    }
                ).collect_view()}
            </CardGrid>
        </Show>
    }
}
