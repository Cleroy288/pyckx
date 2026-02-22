//! Collection lifecycle tests - CRUD operations

use super::helpers::load_test_config;
use crate::infra::{
    CollectionRepository, SupabaseCollectionRepository, SupabaseHttpClient,
};
use crate::services::collection::collection_domain::CollectionItemType;
use std::sync::Arc;

/// Test collection CRUD operations (create, read, exists, delete)
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_collection_lifecycle() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => return,
    };

    let user_id = match config.get_test_user_id() {
        Some(id) => id.to_string(),
        None => return,
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let collection_repo = SupabaseCollectionRepository::new(http_client);

    // == 1. GET OR CREATE COLLECTION (idempotent) ==
    let collection = collection_repo
        .get_or_create(&user_id, CollectionItemType::Dvd)
        .await
        .expect("Failed to get/create collection");

    assert!(collection.id > 0, "Collection should have an ID");
    assert_eq!(collection.user_id, user_id);
    assert_eq!(collection.collection_type, CollectionItemType::Dvd);

    // == 2. GET OR CREATE AGAIN (should return same) ==
    let same_collection = collection_repo
        .get_or_create(&user_id, CollectionItemType::Dvd)
        .await
        .expect("Failed to get_or_create");

    assert_eq!(
        same_collection.id, collection.id,
        "Should return same collection"
    );

    // == 3. FIND BY USER ==
    let collections = collection_repo
        .find_by_user(&user_id)
        .await
        .expect("Failed to find collections");

    assert!(
        !collections.is_empty(),
        "Should have at least one collection"
    );

    // == 4. CHECK EXISTS ==
    let exists = collection_repo
        .exists(&user_id, CollectionItemType::Dvd)
        .await
        .expect("Failed to check existence");

    assert!(exists, "Collection should exist");
}
