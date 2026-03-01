//! AppService user_has_app integration tests

mod helpers;

use helpers::factories::{test_app, test_user_app};
use helpers::stub_app_repo::StubAppRepository;
use helpers::stub_user_app_repo::StubUserAppRepository;
use std::sync::Arc;
use LAPP::services::app_registry::registry_service::AppService;

/// Build service with user-1 having collection app
fn make_service() -> AppService {
    let apps = vec![test_app(1, "collection"), test_app(2, "intello")];
    let user_apps = vec![test_user_app(1, "user-1", 1)];
    AppService::new(
        Arc::new(StubAppRepository::with_apps(apps)),
        Arc::new(StubUserAppRepository::with_user_apps(user_apps)),
    )
}

#[tokio::test]
async fn test_user_has_app_present_returns_true() {
    // arrange
    let svc = make_service();

    // act
    let has = svc.user_has_app("user-1", "collection").await.unwrap();

    // assert
    assert!(has);
}

#[tokio::test]
async fn test_user_has_app_absent_returns_false() {
    // arrange
    let svc = make_service();

    // act - user-1 doesn't have intello
    let has = svc.user_has_app("user-1", "intello").await.unwrap();

    // assert
    assert!(!has);
}

#[tokio::test]
async fn test_user_has_app_nonexistent_app_propagates_error() {
    // arrange - app "quiz" not in DB
    let svc = AppService::new(
        Arc::new(StubAppRepository::new()),
        Arc::new(StubUserAppRepository::new()),
    );

    // act
    let result = svc.user_has_app("user-1", "quiz").await;

    // assert
    assert!(result.is_err());
}
