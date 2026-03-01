use crate::services::app_registry::registry_domain::AppModule;
use crate::services::collection::app_domain::CollectionApp;
use crate::services::collection::collection_domain::CollectionItemType;

#[test]
fn test_collection_app_info() {
    let app = CollectionApp::new();
    assert_eq!(app.info().id, "collection");
    assert_eq!(app.info().name, "Collection");
}

#[test]
fn test_available_item_types() {
    let app = CollectionApp::new();
    let types = app.available_item_types();

    assert!(!types.is_empty());
    assert!(types.contains(&CollectionItemType::Dvd));
}

#[test]
fn test_supports_item_type() {
    let app = CollectionApp::new();

    assert!(app.supports_item_type(&CollectionItemType::Dvd));
    // Book not yet supported
    assert!(!app.supports_item_type(&CollectionItemType::Book));
}
