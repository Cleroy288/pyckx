//! Reusable reactive hooks for common patterns

use dioxus::prelude::*;
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
/// show errors via log
pub fn use_fetch_on_mount<T: 'static>(
    data: Signal<T>,
    loading: Signal<bool>,
    fetch: FetchFn<T>,
) {
    use_effect(move || {
        let fut = fetch();
        spawn(async move {
            let mut d = data;
            let mut l = loading;
            match fut.await {
                Ok(val) => d.set(val),
                Err(e) => {
                    web_sys::console::error_1(
                        &e.into(),
                    );
                }
            }
            l.set(false);
        });
    });
}

/// Derive a bool memo: true when vec is non-empty
pub fn has_items<T: PartialEq + Clone + 'static>(
    items: Signal<Vec<T>>,
) -> Memo<bool> {
    use_memo(move || !(items)().is_empty())
}

/// Navigate to a path from within an async context.
/// Returns a signal — set it to trigger navigation.
pub fn use_async_navigate() -> Signal<Option<String>>
{
    let nav = navigator();
    let target: Signal<Option<String>> =
        use_signal(|| None);
    use_effect(move || {
        if let Some(path) = (target)() {
            nav.push(&*path);
            let mut t = target;
            t.set(None);
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
    use_effect(move || {
        let path = current_pathname()
            .unwrap_or_default();
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
    let nav = navigator();
    use_effect(move || {
        let checking =
            (auth.is_checking_session)();
        let has_user = (auth.user)().is_some();
        if !checking && has_user {
            let target =
                crate::state::palette_dom::read_last_route()
                    .unwrap_or_else(|| {
                        "/home".to_string()
                    });
            nav.push(&*target);
        }
    });
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

/// Read the current pathname from browser location
fn current_pathname() -> Option<String> {
    web_sys::window()?
        .location()
        .pathname()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

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
