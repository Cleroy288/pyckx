//! IntelloService QCM update integration tests

mod helpers;

use helpers::factories::test_qcm_set;
use helpers::stub_intello::make_intello_service;

#[tokio::test]
async fn test_update_qcm_set_owned_succeeds() {
    // arrange
    let svc = make_intello_service();
    let mut set = svc
        .create_qcm_set(test_qcm_set("user-1", 2))
        .await
        .unwrap();
    set.name = "Updated Name".to_string();

    // act
    let updated = svc.update_qcm_set(set).await.unwrap();

    // assert
    assert!(updated);
}

#[tokio::test]
async fn test_update_qcm_set_wrong_user_returns_false() {
    // arrange
    let svc = make_intello_service();
    let mut set = svc
        .create_qcm_set(test_qcm_set("user-1", 2))
        .await
        .unwrap();
    set.user_id = "user-other".into(); // wrong user

    // act
    let updated = svc.update_qcm_set(set).await.unwrap();

    // assert
    assert!(!updated);
}

#[tokio::test]
async fn test_update_qcm_set_nonexistent_returns_false() {
    // arrange
    let svc = make_intello_service();
    let set = test_qcm_set("user-1", 2); // never created

    // act
    let updated = svc.update_qcm_set(set).await.unwrap();

    // assert
    assert!(!updated);
}

#[tokio::test]
async fn test_update_qcm_set_validates_before_ownership() {
    // arrange
    let svc = make_intello_service();
    let mut set = test_qcm_set("user-1", 1);
    set.name = String::new(); // invalid

    // act
    let result = svc.update_qcm_set(set).await;

    // assert - validation error, not ownership check
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_qcm_set_persists_changes() {
    // arrange
    let svc = make_intello_service();
    let mut set = svc
        .create_qcm_set(test_qcm_set("user-1", 1))
        .await
        .unwrap();
    let set_id = set.id.clone();
    set.name = "New Name".to_string();
    svc.update_qcm_set(set).await.unwrap();

    // act - re-read
    let found = svc
        .get_qcm_set(set_id.as_str(), "user-1")
        .await
        .unwrap()
        .unwrap();

    // assert
    assert_eq!(found.name, "New Name");
}
