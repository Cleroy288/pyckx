//! QCM ownership tests - ownership verification and cleanup

use super::helpers::{create_test_service, get_test_user_id};
use crate::domain::intello::{Level, QcmSet};

// ============================================================================
// OWNERSHIP VERIFICATION TESTS (Requirements 5.2, 5.3)
// ============================================================================

#[tokio::test]
async fn test_update_qcmset_ownership_verification() {
    let service = create_test_service();
    let owner_id = "owner-user-id".to_string();
    let attacker_id = "attacker-user-id".to_string();

    let set_id = uuid::Uuid::new_v4().to_string();
    let qcmset = QcmSet {
        id: set_id.clone(),
        user_id: owner_id.clone(),
        name: "Owner's Set".to_string(),
        description: "This belongs to owner".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let _ = service.create_qcm_set(qcmset).await;

    let attacker_update = QcmSet {
        id: set_id.clone(),
        user_id: attacker_id.clone(),
        name: "Hacked Name".to_string(),
        description: "Hacked description".to_string(),
        level: Level::Hard,
        subjects: vec!["python".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let result = service.update_qcm_set(attacker_update).await;
    assert!(result.is_ok(), "Should not error");
    assert!(!result.unwrap(), "Should return false when user doesn't own the set");

    let original = service.get_qcm_set(&set_id, &owner_id).await.unwrap().unwrap();
    assert_eq!(original.name, "Owner's Set", "Original name should be unchanged");

    let _ = service.delete_qcm_set(&set_id, &owner_id).await;
}

#[tokio::test]
async fn test_delete_qcmset_ownership_verification() {
    let service = create_test_service();
    let owner_id = "delete-owner-id".to_string();
    let attacker_id = "delete-attacker-id".to_string();

    let set_id = uuid::Uuid::new_v4().to_string();
    let qcmset = QcmSet {
        id: set_id.clone(),
        user_id: owner_id.clone(),
        name: "Owner's Set to Delete".to_string(),
        description: "This belongs to owner".to_string(),
        level: Level::Easy,
        subjects: vec!["rust".to_string()],
        language: "en".to_string(),
        questions: vec![],
    };

    let _ = service.create_qcm_set(qcmset).await;

    let result = service.delete_qcm_set(&set_id, &attacker_id).await;
    assert!(result.is_ok(), "Should not error");
    assert!(!result.unwrap(), "Should return false when user doesn't own the set");

    let original = service.get_qcm_set(&set_id, &owner_id).await.unwrap();
    assert!(original.is_some(), "Original set should still exist after failed delete attempt");

    let _ = service.delete_qcm_set(&set_id, &owner_id).await;
}

// ============================================================================
// CLEANUP TEST
// ============================================================================

/// Cleanup test - runs last to clean up any test data
#[tokio::test]
async fn z_cleanup_test_qcmsets() {
    let service = create_test_service();
    let user_id = get_test_user_id();

    let sets = match service.get_user_qcm_sets(&user_id).await {
        Ok(s) => s,
        Err(_) => return,
    };

    for set in sets {
        let _ = service.delete_qcm_set(&set.id, &user_id).await;
    }

    let remaining = service.get_user_qcm_sets(&user_id).await.unwrap_or_default();
    assert!(remaining.is_empty(), "All test QCM sets should be deleted");
}
