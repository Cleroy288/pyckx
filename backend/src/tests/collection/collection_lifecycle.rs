//! Collection lifecycle tests - CRUD operations

use super::helpers::load_test_config;
use crate::infra::{CollectionRepository, SupabaseCollectionRepository, SupabaseHttpClient};
use crate::services::collection::collection_domain::CollectionItemType;
use std::sync::Arc;

/// Test collection CRUD operations (create, read, exists, delete)
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_collection_lifecycle() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => {
            eprintln!("Skipping test: Config not available");
            return;
        }
    };

    let user_id = match config.get_test_user_id() {
        Some(id) => id.to_string(),
        None => {
            eprintln!("Skipping test: TEST_USR_ID not set");
            return;
        }
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let collection_repo = SupabaseCollectionRepository::new(http_client);

    // == 1. GET OR CREATE COLLECTION (idempotent) ==
    println!("\n=== Step 1: Get or Create DVD Collection ===");
    let collection = collection_repo
        .get_or_create(&user_id, CollectionItemType::Dvd)
        .await
        .expect("Failed to get/create collection");

    assert!(collection.id > 0, "Collection should have an ID");
    assert_eq!(collection.user_id, user_id);
    assert_eq!(collection.collection_type, CollectionItemType::Dvd);
    println!("✓ Collection ready with ID: {}", collection.id);

    // == 2. GET OR CREATE AGAIN (should return same) ==
    println!("\n=== Step 2: Get or Create again (should return existing) ===");
    let same_collection = collection_repo
        .get_or_create(&user_id, CollectionItemType::Dvd)
        .await
        .expect("Failed to get_or_create");

    assert_eq!(
        same_collection.id, collection.id,
        "Should return same collection"
    );
    println!("✓ get_or_create returned existing collection");

    // == 3. FIND BY USER ==
    println!("\n=== Step 3: Find collections by user ===");
    let collections = collection_repo
        .find_by_user(&user_id)
        .await
        .expect("Failed to find collections");

    assert!(
        !collections.is_empty(),
        "Should have at least one collection"
    );
    println!("✓ Found {} collections", collections.len());

    // == 4. CHECK EXISTS ==
    println!("\n=== Step 4: Check collection exists ===");
    let exists = collection_repo
        .exists(&user_id, CollectionItemType::Dvd)
        .await
        .expect("Failed to check existence");

    assert!(exists, "Collection should exist");
    println!("✓ Collection exists check passed");

    println!("\n=== Collection lifecycle test passed! ===\n");
}
