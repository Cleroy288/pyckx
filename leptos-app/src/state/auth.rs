//! Auth State - Global authentication state management

use crate::api;
use crate::api::AuthResponse;
use leptos::prelude::*;
use leptos::task::spawn_local;

/// Global authentication state
#[derive(Clone, Copy)]
pub struct AuthState {
    /// Current logged-in user (None if not authenticated)
    pub user: RwSignal<Option<AuthResponse>>,
    /// Loading state for auth operations
    pub is_loading: RwSignal<bool>,
    /// Whether we're checking the session on initial load
    pub is_checking_session: RwSignal<bool>,
    /// Error message from last auth operation
    pub error: RwSignal<Option<String>>,
}

impl AuthState {
    /// Create a new auth state
    pub fn new() -> Self {
        Self {
            user: RwSignal::new(None),
            is_loading: RwSignal::new(false),
            is_checking_session: RwSignal::new(true), // Start as true - we'll check on mount
            error: RwSignal::new(None),
        }
    }

    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.user.get().is_some()
    }

    /// Set the current user
    pub fn set_user(&self, user: AuthResponse) {
        self.user.set(Some(user));
        self.error.set(None);
    }

    /// Clear the current user (logout)
    pub fn clear_user(&self) {
        self.user.set(None);
    }

    /// Set error message
    pub fn set_error(&self, error: String) {
        self.error.set(Some(error));
    }

    /// Clear error message
    pub fn clear_error(&self) {
        self.error.set(None);
    }
}

impl Default for AuthState {
    fn default() -> Self {
        Self::new()
    }
}

/// Auth context provider component
/// Wraps children with auth state context
/// Checks session validity on mount
#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let auth_state = AuthState::new();
    provide_context(auth_state);

    // Check session on mount - wrap in Effect for proper lifecycle tracking
    Effect::new(move |_| {
        spawn_local(async move {
            if let Some(user) = api::get_me().await {
                auth_state.set_user(user);
            }
            auth_state.is_checking_session.set(false);
        });
    });

    children()
}

/// Hook to get auth state from context
pub fn use_auth() -> AuthState {
    use_context::<AuthState>().expect(
        "AuthState not found. Wrap with AuthProvider.",
    )
}

/// Redirects to /login if not authenticated.
/// Returns the auth state for further use.
pub fn use_auth_guard() -> AuthState {
    let auth = use_auth();
    let nav = leptos_router::hooks::use_navigate();
    Effect::new(move |_| {
        if !auth.is_checking_session.get()
            && !auth.is_authenticated()
        {
            nav("/login", Default::default());
        }
    });
    auth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::auth_types::AuthResponse;

    /// Factory for test AuthResponse
    fn test_user() -> AuthResponse {
        AuthResponse {
            username: "alice".into(),
            email: "alice@test.com".into(),
            role: "user".into(),
        }
    }

    #[test]
    fn test_new_not_authenticated() {
        let state = AuthState::new();
        assert!(!state.is_authenticated());
    }

    #[test]
    fn test_set_user_makes_authenticated() {
        let state = AuthState::new();
        state.set_user(test_user());
        assert!(state.is_authenticated());
    }

    #[test]
    fn test_set_user_clears_error() {
        let state = AuthState::new();
        state.set_error("oops".into());
        state.set_user(test_user());
        assert!(state.error.get().is_none());
    }

    #[test]
    fn test_clear_user_removes_auth() {
        let state = AuthState::new();
        state.set_user(test_user());
        state.clear_user();
        assert!(!state.is_authenticated());
    }

    #[test]
    fn test_set_error_stores_message() {
        let state = AuthState::new();
        state.set_error("fail".into());
        assert_eq!(
            state.error.get(),
            Some("fail".to_string()),
        );
    }

    #[test]
    fn test_clear_error_removes_message() {
        let state = AuthState::new();
        state.set_error("fail".into());
        state.clear_error();
        assert!(state.error.get().is_none());
    }

    #[test]
    fn test_default_same_as_new() {
        let state = AuthState::default();
        assert!(!state.is_authenticated());
        assert!(state.error.get().is_none());
    }
}

