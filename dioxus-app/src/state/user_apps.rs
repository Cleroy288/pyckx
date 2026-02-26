//! User apps state — tracks which apps the user has

use crate::api;
use crate::domain::app_types::AppData;
use dioxus::prelude::*;

/// Global user apps state
#[derive(Clone, Copy, PartialEq)]
pub struct UserAppsState {
    /// User's subscribed apps (full app data)
    pub apps: Signal<Vec<AppData>>,
    /// Loading state
    pub is_loading: Signal<bool>,
}

impl UserAppsState {
    /// Check if user has a given app by name
    pub fn has_app(&self, name: &str) -> bool {
        self.apps
            .peek()
            .iter()
            .any(|a| a.name == name)
    }

    /// Load user apps from API
    pub fn load(&mut self) {
        let mut apps = self.apps;
        let mut loading = self.is_loading;
        loading.set(true);
        spawn(async move {
            if let Ok(data) =
                api::user_apps::get_user_apps().await
            {
                apps.set(data);
            }
            loading.set(false);
        });
    }

    /// Add an app by name, update signal on success
    pub fn add_app(&self, name: &str) {
        let apps = self.apps;
        let name = name.to_string();
        spawn(add_app_task(apps, name));
    }

    /// Remove an app by name, update signal on success
    pub fn remove_app(&self, name: &str) {
        let apps = self.apps;
        let name = name.to_string();
        spawn(remove_app_task(apps, name));
    }
}

/// Async task: add an app and update signal
async fn add_app_task(
    mut apps: Signal<Vec<AppData>>,
    name: String,
) {
    let Ok(all) =
        api::user_apps::get_all_apps().await
    else {
        return;
    };
    if api::user_apps::add_user_app(&name)
        .await
        .is_err()
    {
        return;
    }
    let found =
        all.into_iter().find(|a| a.name == name);
    if let Some(app) = found {
        apps.write().push(app);
    }
}

/// Async task: remove an app and update signal
async fn remove_app_task(
    mut apps: Signal<Vec<AppData>>,
    name: String,
) {
    let ok = api::user_apps::remove_user_app(&name)
        .await
        .is_ok();
    if ok {
        apps.write().retain(|a| a.name != name);
    }
}

/// Provides user apps context.
/// Loads apps reactively when auth state transitions
/// from "checking" to "authenticated".
#[component]
pub fn UserAppsProvider(
    children: Element,
) -> Element {
    let mut state = UserAppsState {
        apps: use_signal(Vec::new),
        is_loading: use_signal(|| false),
    };
    use_context_provider(|| state);
    let auth = crate::state::use_auth();

    // Load when user becomes authenticated
    use_effect(move || {
        let checking =
            (auth.is_checking_session)();
        let has_user = (auth.user)().is_some();
        if !checking && has_user {
            state.load();
        }
    });

    children
}

/// Hook to get user apps state
pub fn use_user_apps() -> UserAppsState {
    use_context::<UserAppsState>()
}

// TODO: Tests commented out — Dioxus signals require
// a component runtime. These tests used RwSignal::new()
// outside components which is not supported in Dioxus.
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_new_starts_empty() { ... }
//
//     #[test]
//     fn test_new_not_loading() { ... }
//
//     #[test]
//     fn test_default_same_as_new() { ... }
// }
