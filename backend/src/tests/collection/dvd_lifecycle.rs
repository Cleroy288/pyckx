//! DVD lifecycle tests - full CRUD operations

use super::helpers::{create_test_dvd, load_test_config};
use crate::infra::{
    CollectionRepository, DvdRepository, SupabaseCollectionRepository,
    SupabaseDvdRepository, SupabaseHttpClient, UpdateDvd,
};
use crate::services::collection::collection_domain::CollectionItemType;
use crate::services::collection::error_domain::CollectionError;
use std::sync::Arc;

/// Full DVD lifecycle test with collection
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_dvd_full_lifecycle() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => return,
    };

    let user_id = match config.get_test_user_id() {
        Some(id) => id.to_string(),
        None => return,
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let collection_repo =
        SupabaseCollectionRepository::new(http_client.clone());
    let dvd_repo = SupabaseDvdRepository::new(http_client);

    let collection = collection_repo
        .get_or_create(&user_id, CollectionItemType::Dvd)
        .await
        .expect("Failed to get/create collection");

    let test_dvd_name = format!("Test DVD {}", uuid::Uuid::new_v4());

    // == 1. ADD DVD ==
    let create_dvd = create_test_dvd(&test_dvd_name, &user_id, collection.id);
    let dvd = dvd_repo
        .insert(&create_dvd)
        .await
        .expect("Failed to insert DVD");

    assert!(!dvd.id.is_empty(), "DVD should have an ID");
    assert_eq!(dvd.name, test_dvd_name);
    assert_eq!(dvd.collection_id, collection.id);
    assert_eq!(dvd.user_id, user_id);

    let dvd_id = dvd.id.to_string();

    // == 2. MODIFY DVD ==
    let update = UpdateDvd::new()
        .with_name(format!("{} (Updated)", test_dvd_name))
        .with_genre("Thriller")
        .with_actors(vec![
            "Leonardo DiCaprio".to_string(),
            "Tom Hardy".to_string(),
            "Ellen Page".to_string(),
        ]);

    let updated_dvd = dvd_repo
        .update(&user_id, &dvd_id, &update)
        .await
        .expect("Failed to update DVD");

    assert!(updated_dvd.name.contains("(Updated)"));
    assert_eq!(updated_dvd.genre, Some("Thriller".to_string()));
    assert!(updated_dvd.actors.contains("Ellen Page"));

    // == 3. GET ALL USER DVDs ==
    let all_user_dvds = dvd_repo
        .find_all(&user_id)
        .await
        .expect("Failed to find all user DVDs");

    assert!(
        !all_user_dvds.is_empty(),
        "User should have at least one DVD"
    );
    let found_in_all = all_user_dvds.iter().any(|d| d.id == dvd_id);
    assert!(found_in_all, "Our test DVD should be in the user's list");

    // == 4. GET ONE DVD BY ID ==
    let found_dvd = dvd_repo
        .find_by_id(&user_id, &dvd_id)
        .await
        .expect("Failed to find DVD by ID");

    assert_eq!(found_dvd.id, dvd_id);
    assert_eq!(found_dvd.user_id, user_id);

    // == 5. GET DVDs BY COLLECTION ==
    let collection_dvds = dvd_repo
        .find_by_collection(collection.id)
        .await
        .expect("Failed to find DVDs by collection");

    assert!(
        !collection_dvds.is_empty(),
        "Collection should have at least one DVD"
    );
    let found_in_collection = collection_dvds.iter().any(|d| d.id == dvd_id);
    assert!(
        found_in_collection,
        "Our test DVD should be in the collection"
    );

    // == 6. DELETE DVD ==
    let deleted = dvd_repo
        .delete(&user_id, &dvd_id)
        .await
        .expect("Failed to delete DVD");

    assert!(deleted, "DVD should be deleted");

    // Verify deletion
    let find_result = dvd_repo.find_by_id(&user_id, &dvd_id).await;
    assert!(
        find_result.is_err(),
        "DVD should not be found after deletion"
    );
    assert!(
        matches!(
            find_result.unwrap_err(),
            CollectionError::DvdNotFound { .. }
        ),
        "Expected DvdNotFound error after deletion"
    );
}
