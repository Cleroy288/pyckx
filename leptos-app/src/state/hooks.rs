//! Reusable reactive hooks for common patterns

use crate::components::ui::toast::ToastState;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::future::Future;
use std::pin::Pin;

/// Fetch callback type — returns a future with
/// Result<T, String>
pub type FetchFn<T> = Box<
    dyn Fn() -> Pin<
        Box<dyn Future<Output = Result<T, String>>>,
    > + 'static,
>;

/// Fetch data once on mount, store in signal,
/// show errors via toast
pub fn use_fetch_on_mount<T: Send + Sync + 'static>(
    data: RwSignal<T>,
    loading: RwSignal<bool>,
    toast: ToastState,
    fetch: FetchFn<T>,
) {
    Effect::new({
        let mut fired = false;
        move |_| {
            if fired {
                return;
            }
            fired = true;
            let fut = fetch();
            spawn_local(async move {
                match fut.await {
                    Ok(d) => data.set(d),
                    Err(e) => toast.error(e),
                }
                loading.set(false);
            });
        }
    });
}

/// Derive a bool signal: true when vec is non-empty
pub fn has_items<T: Send + Sync + 'static>(
    items: RwSignal<Vec<T>>,
) -> Signal<bool> {
    Signal::derive(move || {
        items.with(|v: &Vec<T>| !v.is_empty())
    })
}

/// Navigate to a path from within an async context.
/// Returns a signal — set it to trigger navigation.
pub fn use_async_navigate() -> RwSignal<Option<String>>
{
    let nav = leptos_router::hooks::use_navigate();
    let target: RwSignal<Option<String>> =
        RwSignal::new(None);
    Effect::new(move |_| {
        if let Some(path) = target.get() {
            nav(&path, Default::default());
            target.set(None);
        }
    });
    target
}

/// Authenticated route prefixes to persist
const AUTH_PREFIXES: [&str; 4] = [
    "/home",
    "/intello",
    "/collection",
    "/admin",
];

/// Check if a path is an authenticated route
fn is_authenticated_route(path: &str) -> bool {
    AUTH_PREFIXES
        .iter()
        .any(|prefix| path.starts_with(prefix))
}

/// Save current route to localStorage on change
pub fn use_persist_route() {
    let location =
        leptos_router::hooks::use_location();
    Effect::new(move |_| {
        let path = location.pathname.get();
        if is_authenticated_route(&path) {
            crate::state::palette_dom::write_last_route(
                &path,
            );
        }
    });
}

/// Redirect to last route (or /home) if
/// user is authenticated
pub fn use_redirect_if_authenticated() {
    let auth = crate::state::use_auth();
    let nav = leptos_router::hooks::use_navigate();
    Effect::new(move |_| {
        let checking =
            auth.is_checking_session.get();
        let has_user = auth.user.get().is_some();
        if !checking && has_user {
            let target =
                crate::state::palette_dom::read_last_route()
                    .unwrap_or_else(|| {
                        "/home".to_string()
                    });
            nav(&target, Default::default());
        }
    });
}

/// Logout handler — calls API, clears auth,
/// navigates to "/".
/// Returns an event handler for on:click.
pub fn use_logout() -> impl Fn(leptos::ev::MouseEvent)
    + Clone
    + 'static
{
    let auth = crate::state::use_auth();
    let nav_to = use_async_navigate();
    move |ev: leptos::ev::MouseEvent| {
        ev.prevent_default();
        auth.is_loading.set(true);
        spawn_local(async move {
            match crate::api::logout().await {
                Ok(()) => {
                    auth.clear_user();
                    auth.is_loading.set(false);
                    nav_to.set(Some("/".into()));
                }
                Err(e) => {
                    auth.set_error(format!(
                        "Logout failed: {e}"
                    ));
                    auth.clear_user();
                    auth.is_loading.set(false);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_items_type_compiles() {
        // Verify the function signature compiles
        // (runtime requires leptos reactive runtime)
        fn _check() -> Signal<bool> {
            let sig = RwSignal::new(vec![1, 2, 3]);
            has_items(sig)
        }
    }

    #[test]
    fn test_is_authenticated_route_home() {
        assert!(is_authenticated_route("/home"));
    }

    #[test]
    fn test_is_authenticated_route_intello_sub() {
        assert!(is_authenticated_route(
            "/intello/qcm"
        ));
    }

    #[test]
    fn test_is_authenticated_route_collection() {
        assert!(is_authenticated_route(
            "/collection"
        ));
    }

    #[test]
    fn test_is_authenticated_route_admin() {
        assert!(is_authenticated_route("/admin"));
    }

    #[test]
    fn test_is_authenticated_route_root_false() {
        assert!(!is_authenticated_route("/"));
    }

    #[test]
    fn test_is_authenticated_route_login_false() {
        assert!(!is_authenticated_route("/login"));
    }

    #[test]
    fn test_is_authenticated_route_register_false() {
        assert!(!is_authenticated_route(
            "/register"
        ));
    }
}
