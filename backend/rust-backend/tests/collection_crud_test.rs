//! CollectionService CRUD integration tests

mod helpers;

use helpers::stub_collection_repo::StubCollectionRepository;
use helpers::stub_dvd_repo::StubDvdRepository;
use std::sync::Arc;
use LAPP::services::collection::collection_domain::CollectionItemType;
use LAPP::services::collection::CollectionService;

/// Build a CollectionService with empty stubs
fn make_service() -> CollectionService {
    CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    )
}

#[tokio::test]
async fn test_add_collection_creates_via_repo() {
    // arrange
    let svc = make_service();

    // act
    let coll = svc
        .add_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();

    // assert
    assert_eq!(coll.user_id, "user-1");
    assert_eq!(coll.collection_type, CollectionItemType::Dvd);
}

#[tokio::test]
async fn test_get_user_collections_returns_all() {
    // arrange
    let svc = make_service();
    svc.add_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();
    svc.add_collection("user-1", CollectionItemType::Book)
        .await
        .unwrap();

    // act
    let colls = svc.get_user_collections("user-1").await.unwrap();

    // assert
    assert_eq!(colls.len(), 2);
}

#[tokio::test]
async fn test_get_user_collections_empty_returns_empty() {
    // arrange
    let svc = make_service();

    // act
    let colls = svc.get_user_collections("user-1").await.unwrap();

    // assert
    assert!(colls.is_empty());
}

#[tokio::test]
async fn test_get_or_create_collection_idempotent() {
    // arrange
    let svc = make_service();

    // act - call twice
    let first = svc
        .get_or_create_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();
    let second = svc
        .get_or_create_collection("user-1", CollectionItemType::Dvd)
        .await
        .unwrap();

    // assert - same id
    assert_eq!(first.id, second.id);
}
