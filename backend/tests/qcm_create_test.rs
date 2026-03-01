//! IntelloService QCM create integration tests

mod helpers;

use helpers::factories::test_qcm_set;
use helpers::stub_intello::make_intello_service;

#[tokio::test]
async fn test_create_qcm_set_valid_stores_and_returns() {
    // arrange
    let svc = make_intello_service();
    let set = test_qcm_set("user-1", 3);

    // act
    let created = svc.create_qcm_set(set).await.unwrap();

    // assert
    assert_eq!(created.user_id.as_str(), "user-1");
}

#[tokio::test]
async fn test_create_qcm_set_invalid_fails_before_storing() {
    // arrange
    let svc = make_intello_service();
    let mut set = test_qcm_set("user-1", 1);
    set.name = String::new(); // invalid

    // act
    let result = svc.create_qcm_set(set).await;

    // assert
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_qcm_set_stores_all_questions() {
    // arrange
    let svc = make_intello_service();
    let set = test_qcm_set("user-1", 3);

    // act
    let created = svc.create_qcm_set(set).await.unwrap();

    // assert
    assert_eq!(created.questions.len(), 3);
}

#[tokio::test]
async fn test_create_qcm_set_fields_match_input() {
    // arrange
    let svc = make_intello_service();
    let mut set = test_qcm_set("user-1", 2);
    set.name = "My QCM".to_string();
    set.description = "Test desc".to_string();
    let expected_id = set.id.clone();

    // act
    let created = svc.create_qcm_set(set).await.unwrap();

    // assert
    assert_eq!(created.id, expected_id);
    assert_eq!(created.name, "My QCM");
    assert_eq!(created.description, "Test desc");
}
