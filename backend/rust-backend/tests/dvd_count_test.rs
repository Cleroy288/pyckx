//! DVD count integration tests

mod helpers;

use helpers::stub_collection_repo::StubCollectionRepository;
use helpers::stub_dvd_repo::StubDvdRepository;
use std::sync::Arc;
use LAPP::services::collection::CollectionService;

/// Build service and insert N DVDs
async fn setup_with_n_dvds(n: usize) -> CollectionService {
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );
    for i in 0..n {
        svc.add_dvd(
            "user-1",
            format!("Film {}", i),
            chrono::Utc::now(),
            None,
            vec![],
            None,
        )
        .await
        .unwrap();
    }
    svc
}

#[tokio::test]
async fn test_get_dvd_count_returns_correct_count() {
    // arrange
    let svc = setup_with_n_dvds(5).await;

    // act
    let count = svc.get_dvd_count("user-1").await.unwrap();

    // assert
    assert_eq!(count, 5);
}

#[tokio::test]
async fn test_get_dvd_count_empty_returns_zero() {
    // arrange
    let svc = setup_with_n_dvds(0).await;

    // act
    let count = svc.get_dvd_count("user-1").await.unwrap();

    // assert
    assert_eq!(count, 0);
}

#[tokio::test]
async fn test_get_collection_dvds_returns_expected_count() {
    // arrange
    let svc = setup_with_n_dvds(3).await;

    // act
    let dvds =
        svc.get_collection_dvds("user-1").await.unwrap();

    // assert
    assert_eq!(dvds.len(), 3);
}

#[tokio::test]
async fn test_get_user_dvds_returns_expected_count() {
    // arrange
    let svc = setup_with_n_dvds(3).await;

    // act
    let dvds = svc.get_user_dvds("user-1").await.unwrap();

    // assert
    assert_eq!(dvds.len(), 3);
}
