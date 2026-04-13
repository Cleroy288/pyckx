//! Ownership verification property tests
//!
//! Ensures that QcmSet update and delete operations
//! respect ownership boundaries.

use proptest::prelude::*;

use super::generators::{
    arb_different_user_ids, arb_qcm_set,
    create_test_study_service,
};

proptest! {
    /// Update must be rejected when user_id does not
    /// match the owner.
    #[test]
    fn test_service_ownership_on_update(
        qcm_set in arb_qcm_set(),
        (owner_id, attacker_id) in arb_different_user_ids()
    ) {
        let service = create_test_study_service();
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create runtime");

        rt.block_on(async {
            let mut owned = qcm_set.clone();
            owned.user_id = owner_id.clone().into();
            let created = service
                .create_qcm_set(owned)
                .await
                .expect("Create should succeed");

            let mut attack = created.clone();
            attack.user_id = attacker_id.clone().into();
            attack.name = "Hacked".to_string();
            let update = service
                .update_qcm_set(attack)
                .await;

            match update {
                Ok(true) => prop_assert!(
                    false,
                    "Should NOT succeed for non-owner"
                ),
                _ => {}
            }

            let original = service
                .get_qcm_set(&created.id, &owner_id)
                .await
                .expect("Get should succeed")
                .expect("Should exist");
            prop_assert_eq!(original.name, created.name);
            prop_assert_eq!(
                original.user_id.as_str(), owner_id
            );
            Ok(())
        })?;
    }

    /// Delete must be rejected when user_id does not
    /// match the owner.
    #[test]
    fn test_service_ownership_on_delete(
        qcm_set in arb_qcm_set(),
        (owner_id, attacker_id) in arb_different_user_ids()
    ) {
        let service = create_test_study_service();
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create runtime");

        rt.block_on(async {
            let mut owned = qcm_set.clone();
            owned.user_id = owner_id.clone().into();
            let created = service
                .create_qcm_set(owned)
                .await
                .expect("Create should succeed");

            let delete = service
                .delete_qcm_set(&created.id, &attacker_id)
                .await;

            match delete {
                Ok(true) => prop_assert!(
                    false,
                    "Should NOT succeed for non-owner"
                ),
                _ => {}
            }

            let original = service
                .get_qcm_set(&created.id, &owner_id)
                .await
                .expect("Get should succeed")
                .expect("Should still exist");
            prop_assert_eq!(original.id, created.id);
            prop_assert_eq!(
                original.user_id.as_str(), owner_id
            );
            Ok(())
        })?;
    }
}
