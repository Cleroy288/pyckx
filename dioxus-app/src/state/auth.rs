//! Auth State - Global authentication state management

use crate::api;
use crate::api::AuthResponse;
use dioxus::prelude::*;

/// Global authentication state
#[derive(Clone, Copy, PartialEq)]
pub struct AuthState {
    /// Current logged-in user (None if not authenticated)
    pub user: Signal<Option<AuthResponse>>,
    /// Loading state for auth operations
    pub is_loading: Signal<bool>,
    /// Whether we're checking the session on initial load
    pub is_checking_session: Signal<bool>,
    /// Error message from last auth operation
    pub error: Signal<Option<String>>,
}

impl AuthState {
    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        (self.user)().is_some()
    }

    /// Set the current user
    pub fn set_user(&mut self, user: AuthResponse) {
        self.user.set(Some(user));
        self.error.set(None);
    }

    /// Clear the current user (logout)
    pub fn clear_user(&mut self) {
        self.user.set(None);
    }

    /// Set error message
    pub fn set_error(&mut self, error: String) {
        self.error.set(Some(error));
    }

    /// Clear error message
    pub fn clear_error(&mut self) {
        self.error.set(None);
    }
}

/// Auth context provider component
/// Wraps children with auth state context
/// Checks session validity on mount
#[component]
pub fn AuthProvider(children: Element) -> Element {
    let mut auth_state = AuthState {
        user: use_signal(|| None),
        is_loading: use_signal(|| false),
        is_checking_session: use_signal(|| true),
        error: use_signal(|| None),
    };
    use_context_provider(|| auth_state);

    // Check session on mount
    use_effect(move || {
        spawn(async move {
            if let Some(user) = api::get_me().await {
                auth_state.set_user(user);
            }
            auth_state.is_checking_session.set(false);
        });
    });

    children
}

/// Hook to get auth state from context
pub fn use_auth() -> AuthState {
    use_context::<AuthState>()
}

/// Redirects to /login if not authenticated.
pub fn use_auth_guard() -> AuthState {
    let auth = use_auth();
    let nav = navigator();
    use_effect(move || {
        if !(auth.is_checking_session)()
            && !auth.is_authenticated()
        {
            nav.push("/login");
        }
    });
    auth
}
