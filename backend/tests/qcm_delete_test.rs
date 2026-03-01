//! IntelloService QCM delete integration tests

mod helpers;

use helpers::factories::test_qcm_set;
use helpers::stub_intello::make_intello_service;

#[tokio::test]
async fn test_delete_qcm_set_owned_succeeds() {
    // arrange
    let svc = make_intello_service();
    let created = svc
        .create_qcm_set(test_qcm_set("user-1", 1))
        .await
        .unwrap();

    // act
    let deleted = svc
        .delete_qcm_set(created.id.as_str(), "user-1")
        .await
        .unwrap();

    // assert
    assert!(deleted);
}

#[tokio::test]
async fn test_delete_qcm_set_wrong_user_returns_false() {
    // arrange
    let svc = make_intello_service();
    let created = svc
        .create_qcm_set(test_qcm_set("user-1", 1))
        .await
        .unwrap();

    // act
    let deleted = svc
        .delete_qcm_set(created.id.as_str(), "user-other")
        .await
        .unwrap();

    // assert
    assert!(!deleted);
}

#[tokio::test]
async fn test_delete_qcm_set_nonexistent_returns_false() {
    // arrange
    let svc = make_intello_service();

    // act
    let deleted = svc
        .delete_qcm_set("fake-id", "user-1")
        .await
        .unwrap();

    // assert
    assert!(!deleted);
}

#[tokio::test]
async fn test_delete_qcm_set_removes_from_repo() {
    // arrange
    let svc = make_intello_service();
    let created = svc
        .create_qcm_set(test_qcm_set("user-1", 1))
        .await
        .unwrap();
    let set_id = created.id.clone();

    // act
    svc.delete_qcm_set(set_id.as_str(), "user-1")
        .await
        .unwrap();

    // assert - should be gone
    let found = svc
        .get_qcm_set(set_id.as_str(), "user-1")
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn test_delete_qcm_set_validates_ids() {
    // arrange
    let svc = make_intello_service();

    // act - empty set_id
    let result = svc.delete_qcm_set("", "user-1").await;

    // assert
    assert!(result.is_err());
}
