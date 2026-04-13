//! QCM property-based tests
//!
//! Serialization round-trips, error mapping, repository
//! persistence, and validation for QCM entities.

use proptest::prelude::*;

use super::generators::{
    arb_invalid_level_string, arb_non_empty_string,
    arb_qcm_set, arb_valid_level_string,
    create_test_study_service,
};
use super::helpers::StubQcmRepository;
use crate::http_api::data_transfer_object::CreateQcmSetRequest;
use crate::infra::database::QcmRepository;
use crate::services::error_domain::StudyError;
use crate::services::QcmSet;

/// Strategy for generating arbitrary StudyError variants.
fn arb_study_error() -> impl Strategy<Value = StudyError> {
    prop_oneof![
        (arb_non_empty_string(), arb_non_empty_string())
            .prop_map(|(g, s)| StudyError::game_not_found(g, s)),
        (arb_non_empty_string(), arb_non_empty_string())
            .prop_map(|(f, m)| StudyError::validation(f, m)),
        arb_non_empty_string().prop_map(StudyError::storage),
    ]
}

/// Strategy for QcmSets with invalid fields.
fn arb_invalid_qcm_set() -> impl Strategy<Value = QcmSet> {
    prop_oneof![
        arb_qcm_set_with_empty_name(),
        arb_qcm_set_with_empty_desc(),
        arb_qcm_set_with_bad_questions(),
    ]
}

/// QcmSet with empty name.
fn arb_qcm_set_with_empty_name() -> impl Strategy<Value = QcmSet> {
    use super::generators::*;
    (
        arb_uuid(),
        arb_uuid(),
        prop_oneof![
            Just(String::new()),
            Just("   ".to_string()),
            Just("\t\n".to_string())
        ],
        arb_non_empty_string(),
        arb_level(),
        prop::collection::vec(arb_subject(), 0..3),
        arb_language(),
        prop::collection::vec(arb_qcm_question(), 0..3),
    )
        .prop_map(build_qcm_set)
}

/// QcmSet with empty description.
fn arb_qcm_set_with_empty_desc() -> impl Strategy<Value = QcmSet> {
    use super::generators::*;
    (
        arb_uuid(),
        arb_uuid(),
        arb_non_empty_string(),
        prop_oneof![
            Just(String::new()),
            Just("   ".to_string()),
            Just("\t\n".to_string())
        ],
        arb_level(),
        prop::collection::vec(arb_subject(), 0..3),
        arb_language(),
        prop::collection::vec(arb_qcm_question(), 0..3),
    )
        .prop_map(build_qcm_set)
}

/// QcmSet with questions missing wrong answers.
fn arb_qcm_set_with_bad_questions(
) -> impl Strategy<Value = QcmSet> {
    use super::generators::*;
    (
        arb_uuid(),
        arb_uuid(),
        arb_non_empty_string(),
        arb_non_empty_string(),
        arb_level(),
        prop::collection::vec(arb_subject(), 0..3),
        arb_language(),
        prop::collection::vec(
            arb_qcm_question_short_wrong(), 1..3,
        ),
    )
        .prop_map(build_qcm_set)
}

/// QcmQuestion with fewer than 3 wrong answers.
fn arb_qcm_question_short_wrong(
) -> impl Strategy<Value = crate::services::QcmQuestion> {
    use super::generators::*;
    (
        arb_uuid(),
        arb_non_empty_string(),
        prop::collection::vec(arb_non_empty_string(), 0..3),
        arb_non_empty_string(),
        arb_non_empty_string(),
    )
        .prop_map(|(id, q, wrong, right, expl)| {
            crate::services::QcmQuestion {
                id: id.into(),
                question: q,
                wrong_answers: wrong,
                right_answer: right,
                explanation: expl,
            }
        })
}

/// Shared QcmSet builder from a tuple of generators.
#[allow(clippy::type_complexity)]
fn build_qcm_set(
    t: (
        String,
        String,
        String,
        String,
        crate::services::Level,
        Vec<String>,
        String,
        Vec<crate::services::QcmQuestion>,
    ),
) -> QcmSet {
    QcmSet {
        id: t.0.into(),
        user_id: t.1.into(),
        name: t.2,
        description: t.3,
        level: t.4,
        subjects: t.5,
        language: t.6,
        questions: t.7,
    }
}

proptest! {
    /// QcmSet serialization round-trip.
    #[test]
    fn test_qcmset_serialization_roundtrip(
        qcm_set in arb_qcm_set()
    ) {
        let json = serde_json::to_string(&qcm_set)
            .expect("should serialize");
        let back: QcmSet = serde_json::from_str(&json)
            .expect("should deserialize");
        prop_assert_eq!(qcm_set, back);
    }

    /// StudyError status code mapping.
    #[test]
    fn test_study_error_status_code_mapping(
        error in arb_study_error()
    ) {
        let code = error.status();
        match &error {
            StudyError::GameSetNotFound { .. } =>
                prop_assert_eq!(code, 404),
            StudyError::ValidationFailed { .. } =>
                prop_assert_eq!(code, 400),
            StudyError::StorageError { .. } =>
                prop_assert_eq!(code, 500),
            StudyError::ExternalServiceError { .. } =>
                prop_assert_eq!(code, 502),
            StudyError::NotFound =>
                prop_assert_eq!(code, 404),
            StudyError::Forbidden =>
                prop_assert_eq!(code, 403),
            StudyError::Conflict(_) =>
                prop_assert_eq!(code, 409),
        }
    }

    /// Repository persistence round-trip.
    #[test]
    fn test_repository_persistence_roundtrip(
        qcm_set in arb_qcm_set()
    ) {
        let repo = StubQcmRepository::new();
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create runtime");

        rt.block_on(async {
            let inserted = repo.insert(&qcm_set).await
                .expect("Insert should succeed");
            let found = repo
                .find_by_id(&qcm_set.id, &qcm_set.user_id)
                .await
                .expect("Find should succeed")
                .expect("Should be found");
            prop_assert_eq!(qcm_set.clone(), inserted);
            prop_assert_eq!(qcm_set, found);
            Ok(())
        })?;
    }

    /// Service validation rejects invalid QcmSet.
    #[test]
    fn test_service_validation_rejects_invalid_input(
        invalid_set in arb_invalid_qcm_set()
    ) {
        let service = create_test_study_service();
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create runtime");

        rt.block_on(async {
            let result = service
                .create_qcm_set(invalid_set)
                .await;
            prop_assert!(result.is_err());
            match result.unwrap_err() {
                StudyError::ValidationFailed { .. } => {}
                other => prop_assert!(
                    false,
                    "Expected ValidationFailed, got: {:?}",
                    other
                ),
            }
            Ok(())
        })?;
    }

    /// Handler rejects invalid level strings.
    #[test]
    fn test_handler_validation_rejects_invalid_level(
        invalid_level in arb_invalid_level_string(),
        name in arb_non_empty_string(),
        description in arb_non_empty_string(),
    ) {
        let request = CreateQcmSetRequest {
            name,
            description,
            level: invalid_level.clone(),
            subjects: vec!["rust".to_string()],
            language: "en".to_string(),
            questions: vec![],
        };
        let result = request.parse_level();
        prop_assert!(result.is_err());
    }

    /// Handler accepts valid level strings.
    #[test]
    fn test_handler_validation_accepts_valid_levels(
        level_str in arb_valid_level_string(),
    ) {
        let request = CreateQcmSetRequest {
            name: "Test".to_string(),
            description: "Test".to_string(),
            level: level_str.clone(),
            subjects: vec!["rust".to_string()],
            language: "en".to_string(),
            questions: vec![],
        };
        let result = request.parse_level();
        prop_assert!(result.is_ok());
    }
}
