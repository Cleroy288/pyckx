//! User apps state — tracks which apps the user has

use crate::api;
use crate::domain::app_types::AppData;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Global user apps state
#[derive(Clone, Copy)]
pub struct UserAppsState {
    /// User's subscribed apps (full app data)
    pub apps: RwSignal<Vec<AppData>>,
    /// Loading state
    pub is_loading: RwSignal<bool>,
}

impl UserAppsState {
    /// Create a new empty state
    pub fn new() -> Self {
        Self {
            apps: RwSignal::new(Vec::new()),
            is_loading: RwSignal::new(false),
        }
    }

    /// Check if user has a given app by name
    pub fn has_app(&self, name: &str) -> bool {
        self.apps
            .with_untracked(|v| {
                v.iter().any(|a| a.name == name)
            })
    }

    /// Load user apps from API
    pub fn load(&self) {
        let apps = self.apps;
        let loading = self.is_loading;
        loading.set(true);
        spawn_local(async move {
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
        spawn_local(add_app_task(apps, name));
    }

    /// Remove an app by name, update signal on success
    pub fn remove_app(&self, name: &str) {
        let apps = self.apps;
        let name = name.to_string();
        spawn_local(remove_app_task(apps, name));
    }
}

/// Async task: add an app and update signal
async fn add_app_task(
    apps: RwSignal<Vec<AppData>>,
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
        apps.update(|v| v.push(app));
    }
}

/// Async task: remove an app and update signal
async fn remove_app_task(
    apps: RwSignal<Vec<AppData>>,
    name: String,
) {
    let ok = api::user_apps::remove_user_app(&name)
        .await
        .is_ok();
    if ok {
        apps.update(|v| {
            v.retain(|a| a.name != name)
        });
    }
}

impl Default for UserAppsState {
    fn default() -> Self {
        Self::new()
    }
}

/// Provides user apps context.
/// Loads apps reactively when auth state transitions
/// from "checking" to "authenticated".
#[component]
pub fn UserAppsProvider(
    children: Children,
) -> impl IntoView {
    let state = UserAppsState::new();
    provide_context(state);
    let auth = crate::state::use_auth();
    // Load when user becomes authenticated
    Effect::new(move |_| {
        let checking = auth.is_checking_session.get();
        let has_user = auth.user.with(|u| u.is_some());
        if !checking && has_user {
            state.load();
        }
    });
    children()
}

/// Hook to get user apps state
pub fn use_user_apps() -> UserAppsState {
    use_context::<UserAppsState>().expect(
        "UserAppsState not found. Wrap with \
         UserAppsProvider.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_starts_empty() {
        let state = UserAppsState::new();
        assert!(state.apps.get_untracked().is_empty());
    }

    #[test]
    fn test_new_not_loading() {
        let state = UserAppsState::new();
        assert!(!state.is_loading.get_untracked());
    }

    #[test]
    fn test_default_same_as_new() {
        let state = UserAppsState::default();
        assert!(state.apps.get_untracked().is_empty());
        assert!(!state.is_loading.get_untracked());
    }
}
