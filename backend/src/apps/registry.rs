//! App Registry - Central list of all available apps in the platform
//!
//! This is the source of truth for valid apps. When a user tries to add
//! an app to their list, we validate against this registry.

use crate::domain::AppInstance;

// == AVAILABLE APPS REGISTRY // ==

/// All available apps in the platform
pub const AVAILABLE_APPS: &[AppInstance] = &[
    AppInstance::new(
        "collection",
        "Collection",
        "Create and manage collections of items, bookmarks, and resources",
    ),
    AppInstance::new(
        "intello",
        "Intello",
        "Learn and test your knowledge with QCM, flashcards, and other games",
    ),
];

// == REGISTRY FUNCTIONS // ==

/// Check if an app name is valid (exists in the registry)
pub fn is_valid_app(name: &str) -> bool {
    AVAILABLE_APPS.iter().any(|app| app.id == name)
}




