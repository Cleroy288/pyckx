//! MyApps — user's subscribed apps as nav cards

use crate::components::ui::card::{CardGrid, NavCard};
use crate::components::ui::empty_state::EmptyState;
use crate::domain::app_registry::find_app;
use crate::state::user_apps::use_user_apps;
use dioxus::prelude::*;

/// Shows user's apps as clickable nav cards
#[component]
pub fn MyApps() -> Element {
    let state = use_user_apps();
    let nav = use_navigator();

    // Derive list of app info entries
    let entries = use_memo(move || {
        state
            .apps
            .read()
            .iter()
            .filter_map(|a| find_app(&a.name))
            .collect::<Vec<_>>()
    });

    let is_empty =
        use_memo(move || entries().is_empty());

    rsx! {
        if *is_empty.read() {
            EmptyState {
                icon: "Plus".to_string(),
                message: "No apps yet".to_string(),
                description: "Add apps from the \
                    catalog below.".to_string(),
            }
        } else {
            CardGrid {
                for info in entries() {
                    NavCard {
                        title: info.label.to_string(),
                        description: info.description
                            .to_string(),
                        icon: info.icon.to_string(),
                        on_click: {
                            let path = info.path
                                .to_string();
                            let nav = nav.clone();
                            EventHandler::new(move |_| {
                                nav.push(path.clone());
                            })
                        },
                    }
                }
            }
        }
    }
}
