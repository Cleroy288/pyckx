//! AppService remove_user_app integration tests

mod helpers;

use helpers::factories::{test_app, test_user_app};
use helpers::stub_app_repo::StubAppRepository;
use helpers::stub_user_app_repo::StubUserAppRepository;
use std::sync::Arc;
use LAPP::services::app_registry::registry_service::AppService;

/// Build service with registry apps and one user app added
fn make_service_with_user_app() -> AppService {
    let apps = vec![test_app(1, "collection"), test_app(2, "intello")];
    let user_apps = vec![test_user_app(1, "user-1", 1)]; // user-1 has collection
    AppService::new(
        Arc::new(StubAppRepository::with_apps(apps)),
        Arc::new(StubUserAppRepository::with_user_apps(user_apps)),
    )
}

#[tokio::test]
async fn test_remove_user_app_existing_succeeds() {
    // arrange
    let svc = make_service_with_user_app();

    // act
    let result = svc.remove_user_app("user-1", "collection").await;

    // assert
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_remove_user_app_invalid_app_returns_not_found() {
    // arrange
    let svc = make_service_with_user_app();

    // act
    let result = svc.remove_user_app("user-1", "nonexistent").await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_remove_user_app_user_doesnt_have_returns_error() {
    // arrange - user-1 has collection but not intello
    let svc = make_service_with_user_app();

    // act
    let result = svc.remove_user_app("user-1", "intello").await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_remove_user_app_checks_registry_first() {
    // arrange
    let svc = make_service_with_user_app();

    // act - "bogus" is not in registry
    let result = svc.remove_user_app("user-1", "bogus").await;

    // assert - should fail at registry check
    assert!(result.is_err());
}

#[tokio::test]
async fn test_remove_user_app_looks_up_app_id() {
    // arrange - user-1 has collection (app_id=1)
    let svc = make_service_with_user_app();

    // act - remove by name, not by id
    let removed = svc.remove_user_app("user-1", "collection").await.unwrap();

    // assert
    assert!(removed);
}
