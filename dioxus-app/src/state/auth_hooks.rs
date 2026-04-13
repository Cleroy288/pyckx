//! Auth-related hooks — redirect and logout

use dioxus::prelude::*;

use super::nav_hooks::use_async_navigate;

/// Redirect to last route (or last app, or /home)
/// if user is authenticated.
/// Priority: last_route (if app route) > last_app > /home
pub fn use_redirect_if_authenticated() {
    let auth = crate::state::use_auth();
    let nav = navigator();
    use_effect(move || {
        let checking =
            (auth.is_checking_session)();
        let has_user = (auth.user)().is_some();
        if !checking && has_user {
            let target = resolve_login_target();
            nav.push(&*target);
        }
    });
}

/// Determine where to send user after login.
/// Reads pyckx-last-page from localStorage
/// (set by JS interval in App root).
fn resolve_login_target() -> String {
    crate::state::palette_storage::read_last_page()
        .filter(|p| {
            p != "/home"
                && p != "/login"
                && p != "/"
        })
        .unwrap_or_else(|| "/home".to_string())
}

/// Redirects to /home if user is not admin.
/// Returns the auth state for further use.
pub fn use_admin_guard() -> crate::state::auth::AuthState {
    let auth = crate::state::use_auth();
    let nav = navigator();
    use_effect(move || {
        if !(auth.is_checking_session)() {
            let is_admin = (auth.user)()
                .as_ref()
                .is_some_and(|u| u.role == "admin");
            if !is_admin {
                nav.push("/home");
            }
        }
    });
    auth
}

/// Logout handler — calls API, clears auth,
/// navigates to "/".
/// Returns a closure for on:click.
pub fn use_logout() -> impl Fn(Event<MouseData>)
    + Clone
    + 'static
{
    let auth = crate::state::use_auth();
    let nav_to = use_async_navigate();
    move |evt: Event<MouseData>| {
        evt.prevent_default();
        let mut loading = auth.is_loading;
        loading.set(true);
        let auth = auth;
        let nav_to = nav_to;
        spawn(async move {
            let mut loading = auth.is_loading;
            let mut auth = auth;
            match crate::api::logout().await {
                Ok(()) => {
                    auth.clear_user();
                    loading.set(false);
                    let mut n = nav_to;
                    n.set(Some("/".into()));
                }
                Err(e) => {
                    auth.set_error(format!(
                        "Logout failed: {e}"
                    ));
                    auth.clear_user();
                    loading.set(false);
                }
            }
        });
    }
}
