//! App CRUD tests - create, read, update, delete operations

use super::helpers::load_test_config;
use crate::shared::AppError;
use crate::infra::{AppRepository, CreateApp, SupabaseAppRepository, SupabaseHttpClient};
use std::sync::Arc;

/// Test app CRUD operations
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_app_crud_lifecycle() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => {
            eprintln!("Skipping test: Config not available");
            return;
        }
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let repo = SupabaseAppRepository::new(http_client.clone());
    let test_app_name = format!(
        "test_app_{}",
        &uuid::Uuid::new_v4().to_string()[..8]
    );

    // == 1. GET ALL APPS ==
    println!("\n=== Step 1: Get All Apps ===");
    let apps = repo.find_all().await.expect("Failed to get apps");
    println!("✓ Found {} apps", apps.len());

    // == 2. CREATE APP ==
    println!("\n=== Step 2: Create App ===");
    let create = CreateApp::new(&test_app_name, Some("Test app description".to_string()));
    let app = repo.insert(&create).await.expect("Failed to create app");

    assert_eq!(app.name, test_app_name);
    assert_eq!(app.description, Some("Test app description".to_string()));
    println!("✓ App created: {} (id: {})", app.name, app.id);

    // == 3. FIND BY NAME ==
    println!("\n=== Step 3: Find App by Name ===");
    let found = repo
        .find_by_name(&test_app_name)
        .await
        .expect("Failed to find app");
    assert_eq!(found.id, app.id);
    println!("✓ Found app by name: {}", found.name);

    // == 4. UPDATE APP ==
    println!("\n=== Step 4: Update App ===");
    let update = crate::infra::UpdateApp::new().with_description("Updated description");
    let updated = repo
        .update(&test_app_name, &update)
        .await
        .expect("Failed to update app");
    assert_eq!(
        updated.description,
        Some("Updated description".to_string())
    );
    println!(
        "✓ App updated: {}",
        updated.description.unwrap_or_default()
    );

    // == 5. DELETE APP ==
    println!("\n=== Step 5: Delete App ===");
    let deleted = repo
        .delete(&test_app_name)
        .await
        .expect("Failed to delete app");
    assert!(deleted);
    println!("✓ App deleted");

    // Verify deletion
    let find_result = repo.find_by_name(&test_app_name).await;
    assert!(find_result.is_err());
    println!("✓ Verified: App no longer exists");

    println!("\n=== App CRUD tests passed! ===\n");
}

/// Test duplicate app creation fails
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_duplicate_app_creation_fails() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => {
            eprintln!("Skipping test: Config not available");
            return;
        }
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let repo = SupabaseAppRepository::new(http_client);

    let create = CreateApp::new("collection", Some("Duplicate".to_string()));
    let result = repo.insert(&create).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::App(crate::services::app_registry::AppsError::AlreadyExists(name)) => {
            assert_eq!(name, "collection");
            println!("✓ Duplicate app correctly rejected: {}", name);
        }
        other => panic!("Expected AppAlreadyExists error, got: {:?}", other),
    }
}
