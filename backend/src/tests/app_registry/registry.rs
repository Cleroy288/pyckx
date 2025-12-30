//! App registry tests - validation and listing

use crate::services::app_registry::{is_valid_app, AVAILABLE_APPS};

#[test]
fn test_is_valid_app() {
    assert!(is_valid_app("collection"));
    assert!(is_valid_app("intello"));
    assert!(!is_valid_app("invalid_app"));
    assert!(!is_valid_app(""));
}

#[test]
#[allow(clippy::const_is_empty)]
fn test_available_apps_not_empty() {
    assert!(!AVAILABLE_APPS.is_empty());
}

#[test]
fn test_available_apps_contains_expected() {
    let app_ids: Vec<&str> = AVAILABLE_APPS.iter().map(|a| a.id).collect();
    assert!(app_ids.contains(&"collection"));
    assert!(app_ids.contains(&"intello"));
}
