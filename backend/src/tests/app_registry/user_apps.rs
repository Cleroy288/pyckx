//! User apps tests - add/remove apps for users

use super::helpers::load_test_config;
use crate::infra::{
    AppRepository, CreateApp, SupabaseAppRepository, SupabaseHttpClient,
    SupabaseUserAppRepository, UserAppRepository,
};
use crate::shared::AppError;
use std::sync::Arc;

/// Test user app operations (using app_id)
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_user_app_lifecycle() {
    let config = match load_test_config() {
        Some(cfg) => cfg,
        None => return,
    };

    let user_id = match config.get_test_user_id() {
        Some(id) => id.to_string(),
        None => return,
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let app_repo = SupabaseAppRepository::new(http_client.clone());
    let user_app_repo = SupabaseUserAppRepository::new(http_client);

    let test_app_name =
        format!("test_user_app_{}", &uuid::Uuid::new_v4().to_string()[..8]);
    let create =
        CreateApp::new(&test_app_name, Some("Test user app".to_string()));
    let app = app_repo
        .insert(&create)
        .await
        .expect("Failed to create test app");
    let app_id = app.id;

    // == 1. ADD APP TO USER ==
    let user_app = user_app_repo
        .add_app(&user_id, app_id)
        .await
        .expect("Failed to add user app");

    assert_eq!(user_app.user_id, user_id);
    assert_eq!(user_app.app_id, app_id);

    // == 2. CHECK HAS APP ==
    let has_app = user_app_repo
        .has_app(&user_id, app_id)
        .await
        .expect("Failed to check has app");
    assert!(has_app);

    // == 3. TRY ADD DUPLICATE ==
    let duplicate_result = user_app_repo.add_app(&user_id, app_id).await;
    assert!(duplicate_result.is_err());
    match duplicate_result.unwrap_err() {
        AppError::App(
            crate::services::app_registry::AppsError::UserAppAlreadyAdded {
                ..
            },
        ) => {}
        other => panic!("Expected UserAppAlreadyAdded error, got: {:?}", other),
    }

    // == 4. GET USER APPS ==
    let user_apps = user_app_repo
        .find_by_user(&user_id)
        .await
        .expect("Failed to get user apps");

    let found = user_apps.iter().any(|ua| ua.app_id == app_id);
    assert!(found, "Test app should be in user's list");

    // == 5. REMOVE APP FROM USER ==
    let removed = user_app_repo
        .remove_app(&user_id, app_id)
        .await
        .expect("Failed to remove user app");
    assert!(removed);

    // Verify removal
    let has_app = user_app_repo
        .has_app(&user_id, app_id)
        .await
        .expect("Failed to check has app");
    assert!(!has_app);

    // Cleanup
    app_repo
        .delete(&test_app_name)
        .await
        .expect("Failed to cleanup test app");
}
