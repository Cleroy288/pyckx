//! AppService get_user_apps integration tests

mod helpers;

use helpers::factories::{test_app, test_user_app};
use helpers::stub_app_repo::StubAppRepository;
use helpers::stub_user_app_repo::StubUserAppRepository;
use std::sync::Arc;
use LAPP::services::app_registry::registry_service::AppService;

#[tokio::test]
async fn test_get_user_apps_empty_returns_empty() {
    // arrange
    let svc = AppService::new(
        Arc::new(StubAppRepository::new()),
        Arc::new(StubUserAppRepository::new()),
    );

    // act
    let apps = svc.get_user_apps("user-1").await.unwrap();

    // assert
    assert!(apps.is_empty());
}

#[tokio::test]
async fn test_get_user_apps_returns_full_app_objects() {
    // arrange
    let apps = vec![test_app(1, "collection"), test_app(2, "study")];
    let user_apps = vec![
        test_user_app(1, "user-1", 1),
        test_user_app(2, "user-1", 2),
    ];
    let svc = AppService::new(
        Arc::new(StubAppRepository::with_apps(apps)),
        Arc::new(StubUserAppRepository::with_user_apps(user_apps)),
    );

    // act
    let result = svc.get_user_apps("user-1").await.unwrap();

    // assert
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].name, "collection");
    assert_eq!(result[1].name, "study");
}

#[tokio::test]
async fn test_get_user_apps_filters_to_matching_apps() {
    // arrange - user has app_id=99 which doesn't exist in apps table
    let apps = vec![test_app(1, "collection")];
    let user_apps = vec![
        test_user_app(1, "user-1", 1),
        test_user_app(2, "user-1", 99), // orphan
    ];
    let svc = AppService::new(
        Arc::new(StubAppRepository::with_apps(apps)),
        Arc::new(StubUserAppRepository::with_user_apps(user_apps)),
    );

    // act
    let result = svc.get_user_apps("user-1").await.unwrap();

    // assert - orphan app_id=99 is skipped
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "collection");
}

#[tokio::test]
async fn test_get_user_apps_multiple_users_isolated() {
    // arrange
    let apps = vec![test_app(1, "collection"), test_app(2, "study")];
    let user_apps = vec![
        test_user_app(1, "user-A", 1),
        test_user_app(2, "user-B", 2),
    ];
    let svc = AppService::new(
        Arc::new(StubAppRepository::with_apps(apps)),
        Arc::new(StubUserAppRepository::with_user_apps(user_apps)),
    );

    // act
    let a_apps = svc.get_user_apps("user-A").await.unwrap();
    let b_apps = svc.get_user_apps("user-B").await.unwrap();

    // assert
    assert_eq!(a_apps.len(), 1);
    assert_eq!(a_apps[0].name, "collection");
    assert_eq!(b_apps.len(), 1);
    assert_eq!(b_apps[0].name, "study");
}
