//! App CRUD tests - create, read, update, delete operations

use super::helpers::load_test_config;
use crate::infra::{
    AppRepository, CreateApp, SupabaseAppRepository, SupabaseHttpClient,
};
use crate::shared::AppError;
use std::sync::Arc;

/// Test app CRUD operations
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_app_crud_lifecycle() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => return,
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let repo = SupabaseAppRepository::new(http_client.clone());
    let test_app_name =
        format!("test_app_{}", &uuid::Uuid::new_v4().to_string()[..8]);

    // == 1. GET ALL APPS ==
    let _apps = repo.find_all().await.expect("Failed to get apps");

    // == 2. CREATE APP ==
    let create = CreateApp::new(
        &test_app_name,
        Some("Test app description".to_string()),
    );
    let app = repo.insert(&create).await.expect("Failed to create app");

    assert_eq!(app.name, test_app_name);
    assert_eq!(app.description, Some("Test app description".to_string()));

    // == 3. FIND BY NAME ==
    let found = repo
        .find_by_name(&test_app_name)
        .await
        .expect("Failed to find app");
    assert_eq!(found.id, app.id);

    // == 4. UPDATE APP ==
    let update =
        crate::infra::UpdateApp::new().with_description("Updated description");
    let updated = repo
        .update(&test_app_name, &update)
        .await
        .expect("Failed to update app");
    assert_eq!(updated.description, Some("Updated description".to_string()));

    // == 5. DELETE APP ==
    let deleted = repo
        .delete(&test_app_name)
        .await
        .expect("Failed to delete app");
    assert!(deleted);

    // Verify deletion
    let find_result = repo.find_by_name(&test_app_name).await;
    assert!(find_result.is_err());
}

/// Test duplicate app creation fails
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_duplicate_app_creation_fails() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => return,
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let repo = SupabaseAppRepository::new(http_client);

    let create = CreateApp::new("collection", Some("Duplicate".to_string()));
    let result = repo.insert(&create).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::App(
            crate::services::app_registry::AppsError::AlreadyExists(name),
        ) => {
            assert_eq!(name, "collection");
        }
        other => panic!("Expected AppAlreadyExists error, got: {:?}", other),
    }
}
