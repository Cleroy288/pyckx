//! DVD lifecycle tests - focused CRUD operations
//!
//! Each test creates its own data, verifies one behavior,
//! and cleans up. Requires a live Supabase connection.

use super::helpers::{create_test_dvd, load_test_config};
use crate::infra::{
    CollectionRepository, DvdRepository,
    SupabaseCollectionRepository, SupabaseDvdRepository,
    SupabaseHttpClient, UpdateDvd,
};
use crate::services::collection::collection_domain::CollectionItemType;
use crate::services::collection::error_domain::CollectionError;
use std::sync::Arc;

/// Shared setup: config, user_id, repos, collection.
async fn setup() -> Option<(
    String,
    SupabaseDvdRepository,
    i32,
)> {
    let config = load_test_config()?;
    let user_id = config.get_test_user_id()?.to_string();
    let http = Arc::new(SupabaseHttpClient::new(&config));
    let col_repo =
        SupabaseCollectionRepository::new(http.clone());
    let dvd_repo = SupabaseDvdRepository::new(http);

    let collection = col_repo
        .get_or_create(&user_id, CollectionItemType::Dvd)
        .await
        .expect("get/create collection");

    Some((user_id, dvd_repo, collection.id))
}

/// Insert a test DVD and return its id. Caller must delete.
async fn insert_dvd(
    repo: &SupabaseDvdRepository,
    user_id: &str,
    col_id: i32,
) -> String {
    let name = format!("Test DVD {}", uuid::Uuid::new_v4());
    let dvd = create_test_dvd(&name, user_id, col_id);
    let created = repo
        .insert(&dvd)
        .await
        .expect("insert DVD");
    created.id.to_string()
}

#[tokio::test]
#[ignore]
async fn test_insert_dvd_returns_valid_id() {
    // Arrange
    let (user_id, repo, col_id) =
        match setup().await {
            Some(t) => t,
            None => return,
        };
    let name = format!("Test DVD {}", uuid::Uuid::new_v4());
    let dvd = create_test_dvd(&name, &user_id, col_id);

    // Act
    let created = repo.insert(&dvd).await.expect("insert");

    // Assert
    assert!(!created.id.is_empty(), "DVD should have an ID");

    // Cleanup
    let _ = repo.delete(&user_id, &created.id).await;
}

#[tokio::test]
#[ignore]
async fn test_find_by_id_returns_correct_dvd() {
    // Arrange
    let (user_id, repo, col_id) =
        match setup().await {
            Some(t) => t,
            None => return,
        };
    let dvd_id = insert_dvd(&repo, &user_id, col_id).await;

    // Act
    let found = repo
        .find_by_id(&user_id, &dvd_id)
        .await
        .expect("find by id");

    // Assert
    assert_eq!(found.id, dvd_id);

    // Cleanup
    let _ = repo.delete(&user_id, &dvd_id).await;
}

#[tokio::test]
#[ignore]
async fn test_update_dvd_changes_fields() {
    // Arrange
    let (user_id, repo, col_id) =
        match setup().await {
            Some(t) => t,
            None => return,
        };
    let dvd_id = insert_dvd(&repo, &user_id, col_id).await;
    let update = UpdateDvd::new()
        .with_genre("Thriller");

    // Act
    let updated = repo
        .update(&user_id, &dvd_id, &update)
        .await
        .expect("update");

    // Assert
    assert_eq!(
        updated.genre,
        Some("Thriller".to_string()),
    );

    // Cleanup
    let _ = repo.delete(&user_id, &dvd_id).await;
}

#[tokio::test]
#[ignore]
async fn test_find_all_includes_inserted_dvd() {
    // Arrange
    let (user_id, repo, col_id) =
        match setup().await {
            Some(t) => t,
            None => return,
        };
    let dvd_id = insert_dvd(&repo, &user_id, col_id).await;

    // Act
    let all = repo
        .find_all(&user_id)
        .await
        .expect("find all");

    // Assert
    assert!(
        all.iter().any(|d| d.id == dvd_id),
        "Inserted DVD should appear in find_all",
    );

    // Cleanup
    let _ = repo.delete(&user_id, &dvd_id).await;
}

#[tokio::test]
#[ignore]
async fn test_delete_dvd_removes_record() {
    // Arrange
    let (user_id, repo, col_id) =
        match setup().await {
            Some(t) => t,
            None => return,
        };
    let dvd_id = insert_dvd(&repo, &user_id, col_id).await;

    // Act
    let deleted = repo
        .delete(&user_id, &dvd_id)
        .await
        .expect("delete");

    // Assert
    assert!(deleted, "delete should return true");
    let err = repo
        .find_by_id(&user_id, &dvd_id)
        .await
        .unwrap_err();
    assert!(
        matches!(err, CollectionError::DvdNotFound { .. }),
        "Expected DvdNotFound after deletion",
    );
}
