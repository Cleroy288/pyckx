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
    use_context::<AuthState>().expect("AuthState not found in context. Wrap your app with AuthProvider.")
}

