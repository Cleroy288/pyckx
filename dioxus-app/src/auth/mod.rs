//! Auth module — login, register, session state.
//!
//! Single entry point: everything the rest of the app
//! needs (pages, provider, hooks) is re-exported here.

pub mod api;
pub mod login;
pub mod register;
mod shared;
pub mod state;

pub use login::LoginPage;
pub use register::RegisterPage;
pub use state::{
    use_auth, use_auth_guard,
    use_redirect_when_authenticated, AuthProvider,
    AuthState,
};
