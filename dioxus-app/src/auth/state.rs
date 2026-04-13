//! Auth state — global signal, provider, and hooks.

use crate::auth::api;
use crate::domain::auth_types::AuthResponse;
use dioxus::prelude::*;

const HOME_ROUTE: &str = "/home";
const LOGIN_ROUTE: &str = "/login";

/// Reactive authentication state shared across the app.
#[derive(Clone, Copy, PartialEq)]
pub struct AuthState {
    /// Current user — `None` when signed out.
    pub user: Signal<Option<AuthResponse>>,
    /// True while a login / register call is pending.
    pub is_loading: Signal<bool>,
    /// True while the initial `GET /me` is in flight.
    pub is_checking_session: Signal<bool>,
    /// Last error message, if any.
    pub error: Signal<Option<String>>,
}

impl AuthState {
    /// Returns `true` once a user is loaded.
    pub fn is_authenticated(&self) -> bool {
        (self.user)().is_some()
    }

    /// Store the signed-in user and clear any error.
    pub fn set_user(&mut self, user: AuthResponse) {
        self.user.set(Some(user));
        self.error.set(None);
    }

    /// Drop the signed-in user (logout).
    pub fn clear_user(&mut self) {
        self.user.set(None);
    }

    /// Set an error message.
    pub fn set_error(&mut self, message: String) {
        self.error.set(Some(message));
    }

    /// Clear the error message.
    pub fn clear_error(&mut self) {
        self.error.set(None);
    }
}

/// Provider that seeds `AuthState` into the context
/// and checks the current session on mount.
#[component]
pub fn AuthProvider(children: Element) -> Element {
    let mut state = new_state();
    use_context_provider(|| state);
    use_effect(move || {
        spawn(async move {
            load_session(&mut state).await;
        });
    });
    children
}

/// Access the shared `AuthState` from context.
pub fn use_auth() -> AuthState {
    use_context::<AuthState>()
}

/// Redirect to `/login` when the user is not authenticated.
/// Returns the live `AuthState` for further use in the page.
pub fn use_auth_guard() -> AuthState {
    let auth = use_auth();
    let nav = navigator();
    use_effect(move || {
        if should_redirect_to_login(&auth) {
            nav.push(LOGIN_ROUTE);
        }
    });
    auth
}

/// Redirect to `/home` once the user becomes authenticated.
/// Used on `/login` and `/register` pages.
pub fn use_redirect_when_authenticated() {
    let auth = use_auth();
    let nav = navigator();
    use_effect(move || {
        if session_ready_and_signed_in(&auth) {
            nav.push(HOME_ROUTE);
        }
    });
}

/// Build the initial `AuthState` signals.
fn new_state() -> AuthState {
    AuthState {
        user: use_signal(|| None),
        is_loading: use_signal(|| false),
        is_checking_session: use_signal(|| true),
        error: use_signal(|| None),
    }
}

/// Fetch `GET /me` once at startup and update state.
async fn load_session(state: &mut AuthState) {
    if let Some(user) = api::fetch_me().await {
        state.set_user(user);
    }
    state.is_checking_session.set(false);
}

/// `true` when session check finished and no user is set.
fn should_redirect_to_login(auth: &AuthState) -> bool {
    !(auth.is_checking_session)() && !auth.is_authenticated()
}

/// `true` when session check finished and user is set.
fn session_ready_and_signed_in(auth: &AuthState) -> bool {
    !(auth.is_checking_session)() && auth.is_authenticated()
}
