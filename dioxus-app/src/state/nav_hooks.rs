//! Navigation hooks — async navigate, route persist

use dioxus::prelude::*;

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
const AUTH_PREFIXES: [&str; 12] = [
    "/home",
    "/courses",
    "/collection",
    "/admin",
    "/qcm",
    "/flashcards",
    "/true-false",
    "/open-questions",
    "/keywords",
    "/order-phrases",
    "/fill-blanks",
    "/demo",
];

/// Check if a path is an authenticated route
fn is_authenticated_route(path: &str) -> bool {
    AUTH_PREFIXES
        .iter()
        .any(|prefix| path.starts_with(prefix))
}

/// Save current route to localStorage on mount.
/// Captures the path at page load (e.g. refresh).
/// For in-app navigation, callers must use
/// `persist_navigation` explicitly.
pub fn use_persist_route() {
    use_effect(move || {
        let path = current_pathname()
            .unwrap_or_default();
        if is_authenticated_route(&path) {
            crate::state::palette_dom
                ::write_last_route(&path);
        }
    });
}

/// Save a navigation target to localStorage.
/// Call this before `nav.push()` to persist the
/// destination route.
pub fn persist_navigation(path: &str) {
    if is_authenticated_route(path) {
        crate::state::palette_dom::write_last_route(
            path,
        );
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
    fn test_is_authenticated_route_courses() {
        assert!(is_authenticated_route("/courses"));
    }

    #[test]
    fn test_is_authenticated_route_qcm() {
        assert!(is_authenticated_route("/qcm"));
    }

    #[test]
    fn test_is_authenticated_route_flashcards() {
        assert!(is_authenticated_route(
            "/flashcards"
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
