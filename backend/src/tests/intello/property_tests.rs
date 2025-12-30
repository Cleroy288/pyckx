//! Property-based tests for Intello domain entities
//!
//! Uses proptest to verify correctness properties across many random inputs.

use proptest::prelude::*;
use std::sync::Arc;

use super::helpers::{
    StubAiUsageRepository, StubCourseRepository, StubFillBlankRepository, StubFlashcardRepository,
    StubKeywordsRepository, StubOpenQuestionRepository, StubOrderPhraseRepository,
    StubQcmRepository, StubStudySessionRepository, StubTrueOrFalseRepository,
};
use crate::services::intello::qcm_question_domain::QcmQuestion;
use crate::services::intello::qcm_set_domain::QcmSet;
use crate::services::intello::enums_domain::Level;
use crate::services::intello::error_domain::IntelloError;
use crate::http_api::data_transfer_object::intello::CreateQcmSetRequest;
use crate::infra::database::QcmRepository;
use crate::services::{IntelloRepositories, IntelloService, OpenRouterService};
use crate::services::intello::open_question_cache_service::OpenQuestionCache;

/// Create a test IntelloService with stub repositories
fn create_test_intello_service() -> IntelloService {
    let repos = IntelloRepositories {
        qcm_repo: Arc::new(StubQcmRepository::new()),
        ai_qcm_repo: Arc::new(StubQcmRepository::new()),
        open_question_repo: Arc::new(StubOpenQuestionRepository::new()),
        flashcard_repo: Arc::new(StubFlashcardRepository::new()),
        true_false_repo: Arc::new(StubTrueOrFalseRepository),
        keywords_repo: Arc::new(StubKeywordsRepository),
        order_phrase_repo: Arc::new(StubOrderPhraseRepository),
        fill_blank_repo: Arc::new(StubFillBlankRepository),
        course_repo: Arc::new(StubCourseRepository),
        ai_usage_repo: Arc::new(StubAiUsageRepository),
        study_session_repo: Arc::new(StubStudySessionRepository),
    };

    IntelloService::builder()
        .with_repositories(repos)
        .with_openrouter(Arc::new(OpenRouterService::with_google_key(
            String::new(),
            None,
        )))
        .with_cache(Arc::new(OpenQuestionCache::new()))
        .build()
        .expect("Test IntelloService setup should not fail")
}

// == GENERATORS ==

/// Strategy for generating arbitrary Level values
fn arb_level() -> impl Strategy<Value = Level> {
    prop_oneof![Just(Level::Easy), Just(Level::Medium), Just(Level::Hard),]
}

/// Strategy for generating arbitrary subject strings
fn arb_subject() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("rust".to_string()),
        Just("python".to_string()),
        Just("algorithms".to_string()),
        Just("data-structures".to_string()),
        Just("web-development".to_string()),
    ]
}

/// Strategy for generating non-empty strings (for required fields)
/// Ensures at least one non-space character exists
fn arb_non_empty_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9][a-zA-Z0-9 ]{0,99}".prop_map(|s| s.trim().to_string())
}

/// Strategy for generating UUID-like strings
fn arb_uuid() -> impl Strategy<Value = String> {
    "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}"
}

/// Strategy for generating arbitrary QcmQuestion
fn arb_qcm_question() -> impl Strategy<Value = QcmQuestion> {
    (
        arb_uuid(),
        arb_non_empty_string(),
        prop::collection::vec(arb_non_empty_string(), 3..=3),
        arb_non_empty_string(),
        arb_non_empty_string(),
    )
        .prop_map(
            |(id, question, wrong_answers, right_answer, explanation)| QcmQuestion {
                id: id.into(),
                question,
                wrong_answers,
                right_answer,
                explanation,
            },
        )
}

/// Strategy for generating arbitrary language codes
fn arb_language() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("en".to_string()),
        Just("fr".to_string()),
        Just("es".to_string()),
        Just("de".to_string()),
    ]
}

/// Strategy for generating arbitrary QcmSet
fn arb_qcm_set() -> impl Strategy<Value = QcmSet> {
    (
        arb_uuid(),
        arb_uuid(),
        arb_non_empty_string(),
        arb_non_empty_string(),
        arb_level(),
        prop::collection::vec(arb_subject(), 0..3),
        arb_language(),
        prop::collection::vec(arb_qcm_question(), 0..5),
    )
        .prop_map(
            |(id, user_id, name, description, level, subjects, language, questions)| QcmSet {
                id: id.into(),
                user_id: user_id.into(),
                name,
                description,
                level,
                subjects,
                language,
                questions,
            },
        )
}

// == PROPERTY TESTS ==

/// Strategy for generating arbitrary IntelloError variants
fn arb_intello_error() -> impl Strategy<Value = IntelloError> {
    prop_oneof![
        // GameSetNotFound with arbitrary game_type and set_id
        (arb_non_empty_string(), arb_non_empty_string())
            .prop_map(|(game_type, set_id)| IntelloError::game_not_found(game_type, set_id)),
        // ValidationFailed with arbitrary field and message
        (arb_non_empty_string(), arb_non_empty_string())
            .prop_map(|(field, message)| IntelloError::validation(field, message)),
        // StorageError with arbitrary message
        arb_non_empty_string().prop_map(IntelloError::storage),
    ]
}

proptest! {
    /// **Feature: architecture-refactor, Property 1: QcmSet serialization round-trip**
    ///
    /// *For any* valid QcmSet entity, serializing to JSON and deserializing back
    /// SHALL produce an equivalent QcmSet.
    ///
    /// **Validates: Requirements 3.4**
    #[test]
    fn prop_qcmset_serialization_roundtrip(qcm_set in arb_qcm_set()) {
        // Serialize to JSON
        let json = serde_json::to_string(&qcm_set)
            .expect("QcmSet should serialize to JSON");

        // Deserialize back
        let deserialized: QcmSet = serde_json::from_str(&json)
            .expect("JSON should deserialize back to QcmSet");

        // Verify equality
        prop_assert_eq!(qcm_set, deserialized, "Round-trip should preserve QcmSet");
    }

    /// **Feature: architecture-refactor, Property 6: Error to HTTP status code mapping**
    ///
    /// *For any* IntelloError variant, the status_code() method SHALL return:
    /// - 404 for GameSetNotFound
    /// - 400 for ValidationFailed
    /// - 500 for StorageError
    ///
    /// **Validates: Requirements 6.3**
    #[test]
    fn prop_intello_error_status_code_mapping(error in arb_intello_error()) {
        let status_code = error.status();

        match &error {
            IntelloError::GameSetNotFound { .. } => {
                prop_assert_eq!(status_code, 404, "GameSetNotFound should map to 404");
            }
            IntelloError::ValidationFailed { .. } => {
                prop_assert_eq!(status_code, 400, "ValidationFailed should map to 400");
            }
            IntelloError::StorageError { .. } => {
                prop_assert_eq!(status_code, 500, "StorageError should map to 500");
            }
            IntelloError::ExternalServiceError { .. } => {
                prop_assert_eq!(status_code, 502, "ExternalServiceError should map to 502");
            }
            IntelloError::NotFound => {
                prop_assert_eq!(status_code, 404, "NotFound should map to 404");
            }
            IntelloError::Forbidden => {
                prop_assert_eq!(status_code, 403, "Forbidden should map to 403");
            }
            IntelloError::Conflict(_) => {
                prop_assert_eq!(status_code, 409, "Conflict should map to 409");
            }
        }
    }

    /// **Feature: architecture-refactor, Property 2: Repository persistence round-trip**
    ///
    /// *For any* valid QcmSet, inserting via QcmRepository and then finding by ID
    /// SHALL return an equivalent QcmSet.
    ///
    /// **Validates: Requirements 2.3**
    #[test]
    fn prop_repository_persistence_roundtrip(qcm_set in arb_qcm_set()) {
        // Create stub repository for testing
        let repo = StubQcmRepository::new();

        // Use tokio runtime to run async operations
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime");

        rt.block_on(async {
            // Insert the QcmSet
            let inserted = repo.insert(&qcm_set).await
                .expect("Insert should succeed");

            // Find by ID
            let found = repo.find_by_id(&qcm_set.id, &qcm_set.user_id).await
                .expect("Find should succeed")
                .expect("QcmSet should be found after insert");

            // Verify equality
            prop_assert_eq!(qcm_set.clone(), inserted, "Insert should return the same QcmSet");
            prop_assert_eq!(qcm_set, found, "Round-trip should preserve QcmSet");

            Ok(())
        })?;
    }

    /// **Feature: architecture-refactor, Property 3: Service validation rejects invalid input**
    ///
    /// *For any* QcmSet with empty name, empty description, or questions with fewer than 3 wrong answers,
    /// IntelloService.create_qcm_set SHALL return ValidationFailed error.
    ///
    /// **Validates: Requirements 5.1**
    #[test]
    fn prop_service_validation_rejects_invalid_input(invalid_set in arb_invalid_qcm_set()) {
        let service = create_test_intello_service();

        // Use tokio runtime to run async operations
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime");

        rt.block_on(async {
            // Attempt to create the invalid QcmSet
            let result = service.create_qcm_set(invalid_set).await;

            // Should return an error
            prop_assert!(result.is_err(), "Invalid QcmSet should be rejected");

            // Should be a ValidationFailed error
            let err = result.unwrap_err();
            match err {
                IntelloError::ValidationFailed { .. } => {
                    // Expected - validation failed as it should
                }
                other => {
                    prop_assert!(false, "Expected ValidationFailed error, got: {:?}", other);
                }
            }

            Ok(())
        })?;
    }
}

// == GENERATORS FOR INVALID QCMSETS ==

/// Strategy for generating QcmSets with empty name
fn arb_qcm_set_with_empty_name() -> impl Strategy<Value = QcmSet> {
    (
        arb_uuid(),
        arb_uuid(),
        // Empty or whitespace-only name
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
        .prop_map(
            |(id, user_id, name, description, level, subjects, language, questions)| QcmSet {
                id: id.into(),
                user_id: user_id.into(),
                name,
                description,
                level,
                subjects,
                language,
                questions,
            },
        )
}

/// Strategy for generating QcmSets with empty description
fn arb_qcm_set_with_empty_description() -> impl Strategy<Value = QcmSet> {
    (
        arb_uuid(),
        arb_uuid(),
        arb_non_empty_string(),
        // Empty or whitespace-only description
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
        .prop_map(
            |(id, user_id, name, description, level, subjects, language, questions)| QcmSet {
                id: id.into(),
                user_id: user_id.into(),
                name,
                description,
                level,
                subjects,
                language,
                questions,
            },
        )
}

/// Strategy for generating QcmQuestion with fewer than 3 wrong answers
fn arb_qcm_question_with_invalid_wrong_answers() -> impl Strategy<Value = QcmQuestion> {
    (
        arb_uuid(),
        arb_non_empty_string(),
        // 0, 1, or 2 wrong answers (not exactly 3)
        prop::collection::vec(arb_non_empty_string(), 0..3),
        arb_non_empty_string(),
        arb_non_empty_string(),
    )
        .prop_map(
            |(id, question, wrong_answers, right_answer, explanation)| QcmQuestion {
                id: id.into(),
                question,
                wrong_answers,
                right_answer,
                explanation,
            },
        )
}

/// Strategy for generating QcmSets with questions that have fewer than 3 wrong answers
fn arb_qcm_set_with_invalid_questions() -> impl Strategy<Value = QcmSet> {
    (
        arb_uuid(),
        arb_uuid(),
        arb_non_empty_string(),
        arb_non_empty_string(),
        arb_level(),
        prop::collection::vec(arb_subject(), 0..3),
        arb_language(),
        // At least one question with invalid wrong_answers count
        prop::collection::vec(arb_qcm_question_with_invalid_wrong_answers(), 1..3),
    )
        .prop_map(
            |(id, user_id, name, description, level, subjects, language, questions)| QcmSet {
                id: id.into(),
                user_id: user_id.into(),
                name,
                description,
                level,
                subjects,
                language,
                questions,
            },
        )
}

/// Strategy for generating any invalid QcmSet (one of the three invalid types)
fn arb_invalid_qcm_set() -> impl Strategy<Value = QcmSet> {
    prop_oneof![
        arb_qcm_set_with_empty_name(),
        arb_qcm_set_with_empty_description(),
        arb_qcm_set_with_invalid_questions(),
    ]
}

/// Strategy for generating two different user IDs
fn arb_different_user_ids() -> impl Strategy<Value = (String, String)> {
    (arb_uuid(), arb_uuid()).prop_filter("user IDs must be different", |(a, b)| a != b)
}

/// Strategy for generating invalid level strings (not "easy", "medium", or "hard")
fn arb_invalid_level_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9_]{1,20}".prop_filter("must not be a valid level", |s| {
        let lower = s.to_lowercase();
        lower != "easy" && lower != "medium" && lower != "hard"
    })
}

// == OWNERSHIP VERIFICATION PROPERTY TESTS ==

proptest! {
    /// **Feature: architecture-refactor, Property 4: Service ownership verification on update**
    ///
    /// *For any* QcmSet update where the user_id does not match the existing set's owner,
    /// IntelloService.update_qcm_set SHALL return false or error.
    ///
    /// **Validates: Requirements 5.2**
    #[test]
    fn prop_service_ownership_verification_on_update(
        qcm_set in arb_qcm_set(),
        (owner_id, attacker_id) in arb_different_user_ids()
    ) {
        let service = create_test_intello_service();

        // Use tokio runtime to run async operations
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime");

        rt.block_on(async {
            // Create a QcmSet owned by owner_id
            let mut owned_set = qcm_set.clone();
            owned_set.user_id = owner_id.clone().into();

            // Insert the set (owned by owner_id)
            let created = service.create_qcm_set(owned_set.clone()).await
                .expect("Create should succeed for valid QcmSet");

            // Now try to update the set with attacker_id (different user)
            let mut attacker_set = created.clone();
            attacker_set.user_id = attacker_id.clone().into();
            attacker_set.name = "Hacked name".to_string();

            let update_result = service.update_qcm_set(attacker_set).await;

            // The update should either return Ok(false) or an error
            // because the attacker doesn't own the set
            match update_result {
                Ok(false) => {
                    // Expected: ownership verification failed, update rejected
                }
                Ok(true) => {
                    prop_assert!(false, "Update should NOT succeed when user_id doesn't match owner");
                }
                Err(_) => {
                    // Also acceptable: an error was returned
                }
            }

            // Verify the original set is unchanged
            let original = service.get_qcm_set(&created.id, &owner_id).await
                .expect("Get should succeed")
                .expect("Original set should still exist");

            prop_assert_eq!(original.name, created.name, "Original set should be unchanged");
            prop_assert_eq!(original.user_id.as_str(), owner_id, "Owner should remain the same");

            Ok(())
        })?;
    }

    /// **Feature: architecture-refactor, Property 5: Service ownership verification on delete**
    ///
    /// *For any* delete request where the user_id does not match the set's owner,
    /// IntelloService.delete_qcm_set SHALL return false.
    ///
    /// **Validates: Requirements 5.3**
    #[test]
    fn prop_service_ownership_verification_on_delete(
        qcm_set in arb_qcm_set(),
        (owner_id, attacker_id) in arb_different_user_ids()
    ) {
        let service = create_test_intello_service();

        // Use tokio runtime to run async operations
        let rt = tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime");

        rt.block_on(async {
            // Create a QcmSet owned by owner_id
            let mut owned_set = qcm_set.clone();
            owned_set.user_id = owner_id.clone().into();

            // Insert the set (owned by owner_id)
            let created = service.create_qcm_set(owned_set.clone()).await
                .expect("Create should succeed for valid QcmSet");

            // Now try to delete the set with attacker_id (different user)
            let delete_result = service.delete_qcm_set(&created.id, &attacker_id).await;

            // The delete should return Ok(false) because the attacker doesn't own the set
            match delete_result {
                Ok(false) => {
                    // Expected: ownership verification failed, delete rejected
                }
                Ok(true) => {
                    prop_assert!(false, "Delete should NOT succeed when user_id doesn't match owner");
                }
                Err(_) => {
                    // Also acceptable: an error was returned
                }
            }

            // Verify the original set still exists (was not deleted)
            let original = service.get_qcm_set(&created.id, &owner_id).await
                .expect("Get should succeed")
                .expect("Original set should still exist after failed delete attempt");

            prop_assert_eq!(original.id, created.id, "Original set should still exist");
            prop_assert_eq!(original.user_id.as_str(), owner_id, "Owner should remain the same");

            Ok(())
        })?;
    }
}

// == HANDLER VALIDATION PROPERTY TESTS ==

/// Strategy for generating valid level strings in various cases
fn arb_valid_level_string() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("easy".to_string()),
        Just("EASY".to_string()),
        Just("Easy".to_string()),
        Just("medium".to_string()),
        Just("MEDIUM".to_string()),
        Just("Medium".to_string()),
        Just("hard".to_string()),
        Just("HARD".to_string()),
        Just("Hard".to_string()),
    ]
}

proptest! {
    /// **Feature: architecture-refactor, Property 7: Handler validation error response**
    ///
    /// *For any* HTTP request with invalid DTO (invalid enum values for level),
    /// the handler SHALL return HTTP 400 with error details.
    ///
    /// This test validates the DTO parsing layer which is responsible for returning
    /// validation errors that result in HTTP 400 responses.
    ///
    /// **Validates: Requirements 7.3**
    #[test]
    fn prop_handler_validation_rejects_invalid_level(
        invalid_level in arb_invalid_level_string(),
        name in arb_non_empty_string(),
        description in arb_non_empty_string(),
    ) {
        // Create a request with an invalid level
        let request = CreateQcmSetRequest {
            name,
            description,
            level: invalid_level.clone(),
            subjects: vec!["rust".to_string()],
            language: "en".to_string(),
            questions: vec![],
        };

        // Attempt to parse the level
        let result = request.parse_level();

        // Should return an error for invalid level
        prop_assert!(
            result.is_err(),
            "parse_level() should reject invalid level '{}', but got Ok({:?})",
            invalid_level,
            result.ok()
        );

        // Error message should mention the invalid value
        let err_msg = result.unwrap_err();
        prop_assert!(
            err_msg.contains(&invalid_level) || err_msg.to_lowercase().contains("invalid"),
            "Error message should mention the invalid level or indicate invalidity: {}",
            err_msg
        );
    }

    /// **Feature: architecture-refactor, Property 7: Handler validation error response**
    ///
    /// *For any* valid level string (case-insensitive), parse_level() should succeed.
    /// This is the inverse property to ensure our validation is correct.
    ///
    /// **Validates: Requirements 7.3**
    #[test]
    fn prop_handler_validation_accepts_valid_levels(
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
        prop_assert!(
            result.is_ok(),
            "parse_level() should accept valid level '{}', but got Err({})",
            level_str,
            result.err().unwrap_or_default()
        );
    }
}
