//! QCM validation tests - input validation rejection tests

use super::helpers::{create_test_service, get_test_user_id};
use crate::services::intello::{Level, qcm_question_domain::QcmQuestion, qcm_set_domain::QcmSet};

// ============================================================================
// CREATE VALIDATION TESTS (Requirements 5.1)
// ============================================================================

#[tokio::test]
async fn test_create_qcmset_rejects_empty_name() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string().into(),
        user_id: user_id.into(),
        name: "".to_string(),
        description: "Valid description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(result.is_err(), "Should reject QCM set with empty name");

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, .. } => {
            assert_eq!(field, "name", "Error should be on 'name' field");
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_qcmset_rejects_whitespace_name() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string().into(),
        user_id: user_id.into(),
        name: "   ".to_string(),
        description: "Valid description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(
        result.is_err(),
        "Should reject QCM set with whitespace-only name"
    );

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, .. } => {
            assert_eq!(field, "name", "Error should be on 'name' field");
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_qcmset_rejects_empty_description() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string().into(),
        user_id: user_id.into(),
        name: "Valid Name".to_string(),
        description: "".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(
        result.is_err(),
        "Should reject QCM set with empty description"
    );

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, .. } => {
            assert_eq!(
                field, "description",
                "Error should be on 'description' field"
            );
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_qcmset_rejects_question_with_wrong_answer_count() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string().into(),
        user_id: user_id.into(),
        name: "Valid Name".to_string(),
        description: "Valid description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![QcmQuestion {
            id: uuid::Uuid::new_v4().to_string().into(),
            question: "What is Rust?".to_string(),
            wrong_answers: vec!["A".to_string(), "B".to_string()], // Only 2
            right_answer: "A programming language".to_string(),
            explanation: "Rust is a systems programming language".to_string(),
        }],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(
        result.is_err(),
        "Should reject QCM set with wrong answer count"
    );

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, message } => {
            assert_eq!(field, "questions", "Error should be on 'questions' field");
            assert!(
                message.contains("3 wrong answers"),
                "Error message should mention 3 wrong answers"
            );
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_qcmset_rejects_empty_user_id() {
    let service = create_test_service();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string().into(),
        user_id: "".to_string().into(),
        name: "Valid Name".to_string(),
        description: "Valid description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(result.is_err(), "Should reject QCM set with empty user_id");

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, .. } => {
            assert_eq!(field, "user_id", "Error should be on 'user_id' field");
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_qcmset_rejects_empty_set_id() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: "".to_string().into(),
        user_id: user_id.into(),
        name: "Valid Name".to_string(),
        description: "Valid description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(result.is_err(), "Should reject QCM set with empty id");

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, .. } => {
            assert_eq!(field, "id", "Error should be on 'id' field");
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_qcmset_rejects_question_with_empty_text() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string().into(),
        user_id: user_id.into(),
        name: "Valid Name".to_string(),
        description: "Valid description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![QcmQuestion {
            id: uuid::Uuid::new_v4().to_string().into(),
            question: "".to_string(),
            wrong_answers: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            right_answer: "D".to_string(),
            explanation: "Explanation".to_string(),
        }],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(
        result.is_err(),
        "Should reject QCM set with empty question text"
    );

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, message } => {
            assert_eq!(field, "questions", "Error should be on 'questions' field");
            assert!(
                message.contains("question text"),
                "Error message should mention question text"
            );
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

#[tokio::test]
async fn test_create_qcmset_rejects_question_with_empty_right_answer() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let qcmset = QcmSet {
        id: uuid::Uuid::new_v4().to_string().into(),
        user_id: user_id.into(),
        name: "Valid Name".to_string(),
        description: "Valid description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![QcmQuestion {
            id: uuid::Uuid::new_v4().to_string().into(),
            question: "What is Rust?".to_string(),
            wrong_answers: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            right_answer: "".to_string(),
            explanation: "Explanation".to_string(),
        }],
    };

    let result = service.create_qcm_set(qcmset).await;
    assert!(
        result.is_err(),
        "Should reject QCM set with empty right_answer"
    );

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, message } => {
            assert_eq!(field, "questions", "Error should be on 'questions' field");
            assert!(
                message.contains("right answer"),
                "Error message should mention right answer"
            );
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }
}

// ============================================================================
// UPDATE VALIDATION TESTS (Requirements 5.2)
// ============================================================================

#[tokio::test]
async fn test_update_qcmset_rejects_empty_name() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let set_id = uuid::Uuid::new_v4().to_string();
    let qcmset = QcmSet {
        id: set_id.clone().into(),
        user_id: user_id.clone().into(),
        name: "Original Name".to_string(),
        description: "Original description".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let _ = service.create_qcm_set(qcmset).await;

    let updated_qcmset = QcmSet {
        id: set_id.clone().into(),
        user_id: user_id.clone().into(),
        name: "".to_string(),
        description: "Updated description".to_string(),
        level: Level::Hard,
        subjects: vec!["python".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.update_qcm_set(updated_qcmset).await;
    assert!(result.is_err(), "Should reject update with empty name");

    let err = result.unwrap_err();
    match err {
        crate::services::intello::error_domain::IntelloError::ValidationFailed { field, .. } => {
            assert_eq!(field, "name", "Error should be on 'name' field");
        }
        _ => panic!("Expected ValidationFailed error, got {:?}", err),
    }

    let _ = service.delete_qcm_set(&set_id, &user_id).await;
}
