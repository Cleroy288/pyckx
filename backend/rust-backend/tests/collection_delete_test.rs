//! CollectionService delete integration tests

mod helpers;

use helpers::stub_collection_repo::StubCollectionRepository;
use helpers::stub_dvd_repo::StubDvdRepository;
use std::sync::Arc;
use LAPP::services::collection::collection_domain::CollectionItemType;
use LAPP::services::collection::CollectionService;

#[tokio::test]
async fn test_delete_dvd_collection_deletes_dvds_first() {
    // arrange
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );
    svc.add_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();
    svc.add_dvd(
        "user-1",
        "Matrix".into(),
        chrono::Utc::now(),
        None,
        vec![],
        None,
    )
    .await
    .unwrap();

    // act
    let deleted = svc
        .delete_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();

    // assert
    assert!(deleted);
    let dvds = svc.get_user_dvds("user-1").await.unwrap();
    assert!(dvds.is_empty());
}

#[tokio::test]
async fn test_delete_collection_nonexistent_returns_false() {
    // arrange
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );

    // act
    let deleted = svc
        .delete_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();

    // assert
    assert!(!deleted);
}

#[tokio::test]
async fn test_delete_collection_cascading_removes_all_dvds() {
    // arrange
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );
    // Add 3 DVDs
    for name in ["Film A", "Film B", "Film C"] {
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
    svc.delete_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();

    // assert
    let dvds = svc.get_user_dvds("user-1").await.unwrap();
    assert!(dvds.is_empty());
}

#[tokio::test]
async fn test_delete_collection_book_type_skips_dvd_delete() {
    // arrange - create a Book collection (no DVD sub-items)
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );
    svc.add_collection("user-1", CollectionItemType::Book)
        .await
        .unwrap();

    // act
    let deleted = svc
        .delete_collection("user-1", CollectionItemType::Book)
        .await
        .unwrap();

    // assert
    assert!(deleted);
}

#[tokio::test]
async fn test_delete_collection_success_returns_true() {
    // arrange
    let svc = CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    );
    svc.add_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();

    // act
    let deleted = svc
        .delete_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();

    // assert
    assert!(deleted);
}
