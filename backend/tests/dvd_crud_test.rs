//! DVD find/update/delete integration tests

mod helpers;

use helpers::stub_collection_repo::StubCollectionRepository;
use helpers::stub_dvd_repo::StubDvdRepository;
use std::sync::Arc;
use LAPP::services::collection::CollectionService;

/// Build service and insert one DVD, return (service, dvd_id)
async fn setup_with_one_dvd() -> (CollectionService, String) {
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );
    let dvd = svc
        .add_dvd(
            "user-1",
            "Matrix".into(),
            chrono::Utc::now(),
            None,
            vec![],
            None,
        )
        .await
        .unwrap();
    (svc, dvd.id)
}

#[tokio::test]
async fn test_get_user_dvds_returns_all() {
    // arrange
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );
    for name in ["A", "B", "C"] {
        svc.add_dvd(
            "user-1",
            name.into(),
            chrono::Utc::now(),
            None,
            vec![],
            None,
        )
        .await
        .unwrap();
    }

    // act
    let dvds = svc.get_user_dvds("user-1").await.unwrap();

    // assert
    assert_eq!(dvds.len(), 3);
}

#[tokio::test]
async fn test_find_dvd_existing_returns_dvd() {
    // arrange
    let (svc, dvd_id) = setup_with_one_dvd().await;

    // act
    let found = svc.find_dvd("user-1", &dvd_id).await.unwrap();

    // assert
    assert_eq!(found.name, "Matrix");
}

#[tokio::test]
async fn test_find_dvd_nonexistent_propagates_error() {
    // arrange
    let (svc, _) = setup_with_one_dvd().await;

    // act
    let result = svc.find_dvd("user-1", "fake-id").await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_dvd_applies_partial_update() {
    // arrange
    let (svc, dvd_id) = setup_with_one_dvd().await;

    // act - only update name
    let updated = svc
        .update_dvd(
            "user-1",
            &dvd_id,
            Some("Reloaded".into()),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    // assert
    assert_eq!(updated.name, "Reloaded");
}

#[tokio::test]
async fn test_delete_dvd_nonexistent_returns_error() {
    // arrange
    let (svc, _) = setup_with_one_dvd().await;

    // act
    let result = svc.delete_dvd("user-1", "fake-id").await;

    // assert
    assert!(result.is_err());
}
