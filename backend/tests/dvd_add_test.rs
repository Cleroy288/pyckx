//! DVD add integration tests

mod helpers;

use helpers::stub_collection_repo::StubCollectionRepository;
use helpers::stub_dvd_repo::StubDvdRepository;
use std::sync::Arc;
use LAPP::services::collection::CollectionService;

/// Build a fresh CollectionService
fn make_service() -> CollectionService {
    CollectionService::new(
        Arc::new(StubCollectionRepository::new()),
        Arc::new(StubDvdRepository::new()),
    )
}

#[tokio::test]
async fn test_add_dvd_creates_collection_then_inserts() {
    // arrange
    let svc = make_service();

    // act
    let dvd = svc
        .add_dvd(
            "user-1",
            "Inception".into(),
            chrono::Utc::now(),
            Some("Nolan".into()),
            vec!["Leo".into()],
            Some("Sci-Fi".into()),
        )
        .await
        .unwrap();

    // assert - DVD created and collection auto-created
    assert_eq!(dvd.name, "Inception");
    let colls = svc.get_user_collections("user-1").await.unwrap();
    assert_eq!(colls.len(), 1);
}

#[tokio::test]
async fn test_add_dvd_reuses_existing_collection() {
    // arrange
    let svc = make_service();
    svc.add_dvd(
        "user-1",
        "Film A".into(),
        chrono::Utc::now(),
        None,
        vec![],
        None,
    )
    .await
    .unwrap();

    // act - second DVD same user
    svc.add_dvd(
        "user-1",
        "Film B".into(),
        chrono::Utc::now(),
        None,
        vec![],
        None,
    )
    .await
    .unwrap();

    // assert - still only 1 collection
    let colls = svc.get_user_collections("user-1").await.unwrap();
    assert_eq!(colls.len(), 1);
}

#[tokio::test]
async fn test_add_dvd_returns_dvd_with_correct_fields() {
    // arrange
    let svc = make_service();

    // act
    let dvd = svc
        .add_dvd(
            "user-1",
            "Matrix".into(),
            chrono::Utc::now(),
            Some("Wachowski".into()),
            vec![],
            Some("Action".into()),
        )
        .await
        .unwrap();

    // assert
    assert_eq!(dvd.name, "Matrix");
    assert_eq!(dvd.realisator.as_deref(), Some("Wachowski"));
    assert_eq!(dvd.genre.as_deref(), Some("Action"));
}

#[tokio::test]
async fn test_add_dvd_joins_actors_as_csv() {
    // arrange
    let svc = make_service();

    // act
    let dvd = svc
        .add_dvd(
            "user-1",
            "Film".into(),
            chrono::Utc::now(),
            None,
            vec!["Alice".into(), "Bob".into()],
            None,
        )
        .await
        .unwrap();

    // assert
    assert_eq!(dvd.actors, "Alice, Bob");
}
