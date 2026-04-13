//! AppService add_user_app integration tests

mod helpers;

use helpers::factories::test_app;
use helpers::stub_app_repo::StubAppRepository;
use helpers::stub_user_app_repo::StubUserAppRepository;
use std::sync::Arc;
use LAPP::services::app_registry::registry_service::AppService;

/// Build an AppService with both registry apps seeded
fn make_service_with_registry_apps() -> AppService {
    let apps = vec![test_app(1, "collection"), test_app(2, "study")];
    AppService::new(
        Arc::new(StubAppRepository::with_apps(apps)),
        Arc::new(StubUserAppRepository::new()),
    )
}

#[tokio::test]
async fn test_add_user_app_valid_app_succeeds() {
    // arrange
    let svc = make_service_with_registry_apps();

    // act
    let result = svc.add_user_app("user-1", "collection").await;

    // assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_add_user_app_invalid_app_returns_not_found() {
    // arrange
    let svc = make_service_with_registry_apps();

    // act
    let result = svc.add_user_app("user-1", "nonexistent").await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_user_app_app_not_in_db_propagates_error() {
    // arrange - registry knows "collection" but DB is empty
    let svc = AppService::new(
        Arc::new(StubAppRepository::new()),
        Arc::new(StubUserAppRepository::new()),
    );

    // act
    let result = svc.add_user_app("user-1", "collection").await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_user_app_returns_correct_ids() {
    // arrange
    let svc = make_service_with_registry_apps();

    // act
    let ua = svc.add_user_app("user-42", "study").await.unwrap();

    // assert
    assert_eq!(ua.user_id, "user-42");
    assert_eq!(ua.app_id, 2); // study has id=2
}

#[tokio::test]
async fn test_add_user_app_both_registry_apps_work() {
    // arrange
    let svc = make_service_with_registry_apps();

    // act
    let r1 = svc.add_user_app("user-1", "collection").await;
    let r2 = svc.add_user_app("user-1", "study").await;

    // assert
    assert!(r1.is_ok());
    assert!(r2.is_ok());
}
