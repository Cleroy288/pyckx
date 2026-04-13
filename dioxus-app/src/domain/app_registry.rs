//! Static app registry — compile-time app metadata

/// Display metadata for a known app
#[derive(PartialEq)]
pub struct AppInfo {
    /// Backend app name (matches DB)
    pub name: &'static str,
    /// Human-readable label
    pub label: &'static str,
    /// Short description for cards
    pub description: &'static str,
    /// Icon name (Icon component)
    pub icon: &'static str,
    /// Frontend route path
    pub path: &'static str,
}

/// All known apps in the platform
pub const APP_REGISTRY: &[AppInfo] = &[
    AppInfo {
        name: "study",
        label: "Study",
        description: "Organize courses, track progress, and excel.",
        icon: "Book",
        path: "/home",
    },
    AppInfo {
        name: "collection",
        label: "Collection",
        description: "Manage your DVD collection effortlessly.",
        icon: "Disc",
        path: "/collection",
    },
];

/// Look up an app by its backend name
pub fn find_app(name: &str) -> Option<&'static AppInfo> {
    APP_REGISTRY.iter().find(|a| a.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_app_existing() {
        let app = find_app("study");
        assert!(app.is_some());
        assert_eq!(app.unwrap().label, "Study");
    }

    #[test]
    fn test_find_app_unknown() {
        assert!(find_app("nonexistent").is_none());
    }

    #[test]
    fn test_registry_has_two_apps() {
        assert_eq!(APP_REGISTRY.len(), 2);
    }

    #[test]
    fn test_registry_paths_start_with_slash() {
        for app in APP_REGISTRY {
            assert!(
                app.path.starts_with('/'),
                "{} path must start with /",
                app.name,
            );
        }
    }
}
