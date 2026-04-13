//! Shared test data generators (proptest strategies)
//!
//! Reusable strategies for generating random domain
//! entities in property-based tests.

use proptest::prelude::*;
use std::sync::Arc;

use crate::services::games::open_question::open_question_cache_service::OpenQuestionCache;
use crate::services::Level;
use crate::services::QcmQuestion;
use crate::services::QcmSet;
use crate::services::{StudyRepositories, StudyService};

use super::helpers::{
    StubAiUsageRepository, StubCourseRepository,
    StubFillBlankRepository, StubFlashcardRepository,
    StubKeywordsRepository, StubOpenQuestionRepository,
    StubOrderPhraseRepository, StubQcmRepository,
    StubStudySessionRepository, StubTrueOrFalseRepository,
};

/// Creates a test StudyService with stub repositories.
pub fn create_test_study_service() -> StudyService {
    let repos = StudyRepositories {
        qcm_repo: Arc::new(StubQcmRepository::new()),
        ai_qcm_repo: Arc::new(StubQcmRepository::new()),
        open_question_repo: Arc::new(
            StubOpenQuestionRepository::new(),
        ),
        flashcard_repo: Arc::new(StubFlashcardRepository::new()),
        true_false_repo: Arc::new(StubTrueOrFalseRepository),
        keywords_repo: Arc::new(StubKeywordsRepository),
        order_phrase_repo: Arc::new(StubOrderPhraseRepository),
        fill_blank_repo: Arc::new(StubFillBlankRepository),
        course_repo: Arc::new(StubCourseRepository),
        ai_usage_repo: Arc::new(StubAiUsageRepository),
        study_session_repo: Arc::new(
            StubStudySessionRepository,
        ),
    };

    StudyService::builder()
        .with_repositories(repos)
        .with_openrouter(Arc::new(
            crate::infra::openrouter::OpenRouterClient::with_google_key(
                String::new(),
                None,
            ),
        ))
        .with_cache(Arc::new(OpenQuestionCache::new()))
        .build()
        .expect("Test StudyService setup should not fail")
}

/// Strategy for generating arbitrary Level values.
pub fn arb_level() -> impl Strategy<Value = Level> {
    prop_oneof![
        Just(Level::Easy),
        Just(Level::Medium),
        Just(Level::Hard),
    ]
}

/// Strategy for generating arbitrary subject strings.
pub fn arb_subject() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("rust".to_string()),
        Just("python".to_string()),
        Just("algorithms".to_string()),
        Just("data-structures".to_string()),
        Just("web-development".to_string()),
    ]
}

/// Strategy for non-empty strings (required fields).
pub fn arb_non_empty_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9][a-zA-Z0-9 ]{0,99}"
        .prop_map(|s| s.trim().to_string())
}

/// Strategy for generating UUID-like strings.
pub fn arb_uuid() -> impl Strategy<Value = String> {
    "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}"
}

/// Strategy for generating arbitrary language codes.
pub fn arb_language() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("en".to_string()),
        Just("fr".to_string()),
        Just("es".to_string()),
        Just("de".to_string()),
    ]
}

/// Strategy for generating an arbitrary QcmQuestion.
pub fn arb_qcm_question() -> impl Strategy<Value = QcmQuestion> {
    (
        arb_uuid(),
        arb_non_empty_string(),
        prop::collection::vec(arb_non_empty_string(), 3..=3),
        arb_non_empty_string(),
        arb_non_empty_string(),
    )
        .prop_map(
            |(id, question, wrong, right, explanation)| {
                QcmQuestion {
                    id: id.into(),
                    question,
                    wrong_answers: wrong,
                    right_answer: right,
                    explanation,
                }
            },
        )
}

/// Strategy for generating an arbitrary QcmSet.
pub fn arb_qcm_set() -> impl Strategy<Value = QcmSet> {
    (
        arb_uuid(),
        arb_uuid(),
        arb_non_empty_string(),
        arb_non_empty_string(),
        arb_level(),
        prop::collection::vec(arb_subject(), 0..3),
        arb_language(),
        prop::collection::vec(arb_qcm_question(), 1..5),
    )
        .prop_map(
            |(
                id,
                user_id,
                name,
                description,
                level,
                subjects,
                language,
                questions,
            ): (
                String,
                String,
                String,
                String,
                Level,
                Vec<String>,
                String,
                Vec<QcmQuestion>,
            )| {
                QcmSet {
                    id: id.into(),
                    user_id: user_id.into(),
                    name,
                    description,
                    level,
                    subjects,
                    language,
                    questions,
                }
            },
        )
}

/// Strategy for two different user IDs.
pub fn arb_different_user_ids(
) -> impl Strategy<Value = (String, String)> {
    (arb_uuid(), arb_uuid()).prop_filter(
        "user IDs must be different",
        |(a, b)| a != b,
    )
}

/// Strategy for invalid level strings.
pub fn arb_invalid_level_string() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9_]{1,20}".prop_filter(
        "must not be a valid level",
        |s| {
            let lower = s.to_lowercase();
            lower != "easy"
                && lower != "medium"
                && lower != "hard"
        },
    )
}

/// Strategy for valid level strings in various cases.
pub fn arb_valid_level_string() -> impl Strategy<Value = String> {
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
