//! DVD edge case tests - error handling for non-existent DVDs

use super::helpers::load_test_config;
use crate::infra::{DvdRepository, SupabaseDvdRepository, SupabaseHttpClient, UpdateDvd};
use crate::services::collection::error_domain::CollectionError;
use std::sync::Arc;

/// Test that non-existent DVD returns proper error
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_find_nonexistent_dvd() {
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
            eprintln!("Skipping test: TEST_USR_ID not set in config");
            return;
        }
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let repo = SupabaseDvdRepository::new(http_client);
    let fake_id = uuid::Uuid::new_v4().to_string();

    let result = repo.find_by_id(&user_id, &fake_id).await;

    assert!(result.is_err(), "Should return error for non-existent DVD");
    match result.unwrap_err() {
        CollectionError::DvdNotFound { dvd_id } => {
            assert_eq!(dvd_id, fake_id, "Error should contain the DVD ID");
        }
        other => panic!("Expected DvdNotFound error, got: {:?}", other),
    }
}

/// Test update on non-existent DVD
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_update_nonexistent_dvd() {
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
            eprintln!("Skipping test: TEST_USR_ID not set in config");
            return;
        }
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let repo = SupabaseDvdRepository::new(http_client);
    let fake_id = uuid::Uuid::new_v4().to_string();

    let update = UpdateDvd::new().with_name("New Name");
    let result = repo.update(&user_id, &fake_id, &update).await;

    assert!(
        result.is_err(),
        "Should return error for non-existent DVD update"
    );
}

/// Test delete on non-existent DVD returns false
#[tokio::test]
#[ignore] // Requires Supabase connection
async fn test_delete_nonexistent_dvd() {
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
            eprintln!("Skipping test: TEST_USR_ID not set in config");
            return;
        }
    };

    let http_client = Arc::new(SupabaseHttpClient::new(&config));
    let repo = SupabaseDvdRepository::new(http_client);
    let fake_id = uuid::Uuid::new_v4().to_string();

    let result = repo.delete(&user_id, &fake_id).await;

    assert!(result.is_ok(), "Delete should not error");
    assert!(
        !result.unwrap(),
        "Delete should return false for non-existent DVD"
    );
}
