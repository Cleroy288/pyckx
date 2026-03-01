//! AppService CRUD integration tests

mod helpers;

use helpers::factories::test_app;
use helpers::stub_app_repo::StubAppRepository;
use helpers::stub_user_app_repo::StubUserAppRepository;
use std::sync::Arc;
use LAPP::services::app_registry::registry_service::AppService;

/// Build an AppService with given stubs
fn make_service(
    app_repo: StubAppRepository,
    user_app_repo: StubUserAppRepository,
) -> AppService {
    AppService::new(Arc::new(app_repo), Arc::new(user_app_repo))
}

#[tokio::test]
async fn test_get_all_apps_empty_returns_empty_vec() {
    // arrange
    let svc = make_service(StubAppRepository::new(), StubUserAppRepository::new());

    // act
    let apps = svc.get_all_apps().await.unwrap();

    // assert
    assert!(apps.is_empty());
}

#[tokio::test]
async fn test_get_all_apps_returns_seeded_apps() {
    // arrange
    let seeded = vec![test_app(1, "collection"), test_app(2, "intello")];
    let svc = make_service(
        StubAppRepository::with_apps(seeded),
        StubUserAppRepository::new(),
    );

    // act
    let apps = svc.get_all_apps().await.unwrap();

    // assert
    assert_eq!(apps.len(), 2);
}

#[tokio::test]
async fn test_get_app_existing_returns_app() {
    // arrange
    let seeded = vec![test_app(1, "collection")];
    let svc = make_service(
        StubAppRepository::with_apps(seeded),
        StubUserAppRepository::new(),
    );

    // act
    let app = svc.get_app("collection").await.unwrap();

    // assert
    assert_eq!(app.name, "collection");
}

#[tokio::test]
async fn test_get_app_nonexistent_returns_error() {
    // arrange
    let svc = make_service(StubAppRepository::new(), StubUserAppRepository::new());

    // act
    let result = svc.get_app("nonexistent").await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_app_stores_and_returns() {
    // arrange
    let svc = make_service(StubAppRepository::new(), StubUserAppRepository::new());

    // act
    let app = svc
        .create_app("quiz", Some("A quiz app".to_string()))
        .await
        .unwrap();

    // assert
    assert_eq!(app.name, "quiz");
    assert_eq!(app.description.as_deref(), Some("A quiz app"));
}
