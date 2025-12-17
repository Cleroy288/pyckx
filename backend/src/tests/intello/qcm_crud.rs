//! QCM CRUD tests - add, get, update, delete operations

use super::helpers::{create_test_service, get_test_user_id};
use crate::apps::IntelloApp;
use crate::domain::intello::{Level, QcmQuestion, QcmSet};
use crate::domain::AppModule;

// ============================================================================
// UNIT TESTS
// ============================================================================

#[test]
fn test_intello_app_info() {
    let app = IntelloApp::new();
    assert_eq!(app.info().id, "intello");
    assert_eq!(app.info().name, "Intello");
}

#[test]
fn test_intello_app_default() {
    let app = IntelloApp::default();
    assert_eq!(app.id(), "intello");
}

#[test]
fn test_get_available_games() {
    let service = create_test_service();
    let games = service.get_available_games();
    assert_eq!(games.len(), 1);
    assert!(games.iter().any(|g| g.id == "qcm"));
}

// ============================================================================
// QCM CRUD TESTS
// ============================================================================

#[tokio::test]
async fn test_add_and_get_qcmset() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user_id.clone(),
        name: "Rust Basics Test".to_string(),
        description: "Test your Rust knowledge".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string(), "programming".to_string()],
        language: "en".to_string(),
        questions: vec![
            QcmQuestion {
                id: uuid::Uuid::new_v4().to_string(),
                question: "What keyword is used to declare a variable in Rust?".to_string(),
                wrong_answers: vec![
                    "var".to_string(),
                    "const".to_string(),
                    "def".to_string(),
                ],
                right_answer: "let".to_string(),
                explanation: "In Rust, 'let' is used to declare variables.".to_string(),
            },
        ],
    };

    let result = service.create_qcm_set(qcmset.clone()).await;
    assert!(result.is_ok(), "Should add QCM set successfully");

    let user_sets = service.get_user_qcm_sets(&user_id).await;
    assert!(user_sets.is_ok(), "Should retrieve QCM sets successfully");

    let sets = user_sets.unwrap();
    assert!(!sets.is_empty(), "User should have at least one QCM set");

    let found = sets.iter().find(|s| s.id == qcmset.id);
    assert!(found.is_some(), "Should find the created QCM set");

    let found_set = found.unwrap();
    assert_eq!(found_set.name, "Rust Basics Test");
    assert_eq!(found_set.level, Level::Easy);
    assert!(found_set.subjects.contains(&"rust".to_string()));
    assert_eq!(found_set.questions.len(), 1);

    let _ = service.delete_qcm_set(&qcmset.id, &user_id).await;
}

#[tokio::test]
async fn test_get_qcmsets_empty_user() {
    let service = create_test_service();
    let fake_user_id = uuid::Uuid::new_v4().to_string();

    let result = service.get_user_qcm_sets(&fake_user_id).await;
    assert!(result.is_ok(), "Should not error for user with no sets");

    let sets = result.unwrap();
    assert!(sets.is_empty(), "New user should have no QCM sets");
}

#[tokio::test]
async fn test_update_qcmset() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let set_id = uuid::Uuid::new_v4().to_string();
    let qcmset = QcmSet {
        id: set_id.clone(),
        user_id: user_id.clone(),
        name: "Original Name".to_string(),
        description: "Original description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(result.is_ok(), "Should add QCM set successfully");

    let updated_qcmset = QcmSet {
        id: set_id.clone(),
        user_id: user_id.clone(),
        name: "Updated Name".to_string(),
        description: "Updated description".to_string(),
        level: Level::Hard,
        subjects: vec!["python".to_string(), "data-science".to_string()],
        language: "en".to_string(),
        questions: vec![
            QcmQuestion {
                id: uuid::Uuid::new_v4().to_string(),
                question: "What is Python?".to_string(),
                wrong_answers: vec![
                    "A snake".to_string(),
                    "A car".to_string(),
                    "A food".to_string(),
                ],
                right_answer: "A programming language".to_string(),
                explanation: "Python is a high-level programming language.".to_string(),
            },
        ],
    };

    let update_result = service.update_qcm_set(updated_qcmset).await;
    assert!(update_result.is_ok(), "Should update QCM set successfully");
    assert!(update_result.unwrap(), "Should return true for successful update");

    let user_sets = service.get_user_qcm_sets(&user_id).await.unwrap();
    let found = user_sets.iter().find(|s| s.id == set_id);
    assert!(found.is_some(), "Should find the updated QCM set");

    let found_set = found.unwrap();
    assert_eq!(found_set.name, "Updated Name");
    assert_eq!(found_set.description, "Updated description");
    assert_eq!(found_set.level, Level::Hard);
    assert!(found_set.subjects.contains(&"python".to_string()));
    assert_eq!(found_set.questions.len(), 1);

    let _ = service.delete_qcm_set(&set_id, &user_id).await;
}

#[tokio::test]
async fn test_update_nonexistent_qcmset() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string(),
        user_id,
        name: "Non-existent".to_string(),
        description: "This set doesn't exist".to_string(),
        level: Level::Medium,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.update_qcm_set(qcmset).await;
    assert!(result.is_ok(), "Should not error");
    assert!(!result.unwrap(), "Should return false for non-existent set");
}

#[tokio::test]
async fn test_delete_qcmset() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let set_id = uuid::Uuid::new_v4().to_string();
    let qcmset = QcmSet {
        id: set_id.clone(),
        user_id: user_id.clone(),
        name: "To Be Deleted".to_string(),
        description: "This set will be deleted".to_string(),
        level: Level::Medium,
        subjects: vec!["python".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let add_result = service.create_qcm_set(qcmset).await;
    assert!(add_result.is_ok(), "Should add QCM set successfully");

    let sets_before = service.get_user_qcm_sets(&user_id).await.unwrap();
    let exists_before = sets_before.iter().any(|s| s.id == set_id);
    assert!(exists_before, "QCM set should exist before deletion");

    let delete_result = service.delete_qcm_set(&set_id, &user_id).await;
    assert!(delete_result.is_ok(), "Should delete QCM set successfully");
    assert!(delete_result.unwrap(), "Should return true for successful deletion");

    let sets_after = service.get_user_qcm_sets(&user_id).await.unwrap();
    let exists_after = sets_after.iter().any(|s| s.id == set_id);
    assert!(!exists_after, "QCM set should not exist after deletion");
}

#[tokio::test]
async fn test_delete_nonexistent_qcmset() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let fake_id = uuid::Uuid::new_v4().to_string();
    let result = service.delete_qcm_set(&fake_id, &user_id).await;

    assert!(result.is_ok(), "Should not error");
    assert!(!result.unwrap(), "Should return false for non-existent set");
}

#[tokio::test]
async fn test_get_qcm_set_by_id() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let set_id = uuid::Uuid::new_v4().to_string();
    let qcmset = QcmSet {
        id: set_id.clone(),
        user_id: user_id.clone(),
        name: "Test Get By ID".to_string(),
        description: "Testing get_qcm_set method".to_string(),
        level: Level::Medium,
        subjects: vec!["python".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let _ = service.create_qcm_set(qcmset.clone()).await;

    let result = service.get_qcm_set(&set_id, &user_id).await;
    assert!(result.is_ok(), "Should retrieve QCM set successfully");

    let found = result.unwrap();
    assert!(found.is_some(), "Should find the QCM set");

    let found_set = found.unwrap();
    assert_eq!(found_set.id, set_id);
    assert_eq!(found_set.name, "Test Get By ID");
    assert_eq!(found_set.level, Level::Medium);

    let _ = service.delete_qcm_set(&set_id, &user_id).await;
}

#[tokio::test]
async fn test_get_qcm_set_not_found() {
    let service = create_test_service();
    let user_id = get_test_user_id();
    let fake_id = uuid::Uuid::new_v4().to_string();

    let result = service.get_qcm_set(&fake_id, &user_id).await;
    assert!(result.is_ok(), "Should not error for non-existent set");
    assert!(result.unwrap().is_none(), "Should return None for non-existent set");
}
