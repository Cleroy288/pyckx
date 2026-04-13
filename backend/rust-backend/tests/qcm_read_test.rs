//! StudyService QCM read integration tests

mod helpers;

use helpers::factories::test_qcm_set;
use helpers::stub_study::make_study_service;

#[tokio::test]
async fn test_get_user_qcm_sets_empty_returns_empty() {
    // arrange
    let svc = make_study_service();

    // act
    let sets = svc.get_user_qcm_sets("user-1").await.unwrap();

    // assert
    assert!(sets.is_empty());
}

#[tokio::test]
async fn test_get_user_qcm_sets_returns_only_user_sets() {
    // arrange
    let svc = make_study_service();
    svc.create_qcm_set(test_qcm_set("user-A", 1)).await.unwrap();
    svc.create_qcm_set(test_qcm_set("user-B", 1)).await.unwrap();

    // act
    let sets = svc.get_user_qcm_sets("user-A").await.unwrap();

    // assert
    assert_eq!(sets.len(), 1);
    assert_eq!(sets[0].user_id.as_str(), "user-A");
}

#[tokio::test]
async fn test_get_qcm_set_existing_returns_some() {
    // arrange
    let svc = make_study_service();
    let created = svc
        .create_qcm_set(test_qcm_set("user-1", 2))
        .await
        .unwrap();

    // act
    let found = svc
        .get_qcm_set(created.id.as_str(), "user-1")
        .await
        .unwrap();

    // assert
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, created.id);
}

#[tokio::test]
async fn test_get_qcm_set_wrong_user_returns_none() {
    // arrange
    let svc = make_study_service();
    let created = svc
        .create_qcm_set(test_qcm_set("user-1", 1))
        .await
        .unwrap();

    // act - different user
    let found = svc
        .get_qcm_set(created.id.as_str(), "user-other")
        .await
        .unwrap();

    // assert
    assert!(found.is_none());
}

#[tokio::test]
async fn test_get_qcm_set_nonexistent_returns_none() {
    // arrange
    let svc = make_study_service();

    // act
    let found = svc.get_qcm_set("fake-id", "user-1").await.unwrap();

    // assert
    assert!(found.is_none());
}
