//! Intello test helpers

use crate::domain::intello::{FillBlankSet, KeywordSet, OrderPhraseSet, TrueOrFalseSet};
use crate::error::IntelloError;
use crate::infrastructure::{
    GameSetRepository, JsonFlashcardRepository, JsonOpenQuestionRepository,
    JsonQcmRepository,
};
use crate::services::{IntelloService, OpenRouterService};
use crate::shared::OpenQuestionCache;
use async_trait::async_trait;
use std::sync::Arc;

/// Path to test QCM sets JSON file
pub const TEST_QCMSET_FILE: &str = "data/intello/qcmset.json";
/// Path to test AI-generated QCM sets JSON file
pub const TEST_AI_QCMSET_FILE: &str = "data/intello/ai_qcmset.json";
/// Path to test open question sets JSON file
pub const TEST_OPEN_QUESTION_FILE: &str = "data/intello/openquestion.json";
/// Path to test flashcard sets JSON file
pub const TEST_FLASHCARD_FILE: &str = "data/intello/flashcard.json";

// == STUB REPOSITORIES FOR TESTS ==
// These are minimal implementations for repos not used in QCM-focused tests

/// Stub implementation of GameSetRepository<TrueOrFalseSet> for tests
pub struct StubTrueOrFalseRepository;

#[async_trait]
impl GameSetRepository<TrueOrFalseSet> for StubTrueOrFalseRepository {
    async fn insert(&self, set: &TrueOrFalseSet) -> Result<TrueOrFalseSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_id(&self, _: &str, _: &str) -> Result<Option<TrueOrFalseSet>, IntelloError> {
        Ok(None)
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        Ok(vec![])
    }
    async fn delete(&self, _: &str, _: &str) -> Result<bool, IntelloError> {
        Ok(false)
    }
}

/// Stub implementation of GameSetRepository<KeywordSet> for tests
pub struct StubKeywordsRepository;

#[async_trait]
impl GameSetRepository<KeywordSet> for StubKeywordsRepository {
    async fn insert(&self, set: &KeywordSet) -> Result<KeywordSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_id(&self, _: &str, _: &str) -> Result<Option<KeywordSet>, IntelloError> {
        Ok(None)
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<KeywordSet>, IntelloError> {
        Ok(vec![])
    }
    async fn delete(&self, _: &str, _: &str) -> Result<bool, IntelloError> {
        Ok(false)
    }
}

/// Stub implementation of GameSetRepository<OrderPhraseSet> for tests
pub struct StubOrderPhraseRepository;

#[async_trait]
impl GameSetRepository<OrderPhraseSet> for StubOrderPhraseRepository {
    async fn insert(&self, set: &OrderPhraseSet) -> Result<OrderPhraseSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_id(&self, _: &str, _: &str) -> Result<Option<OrderPhraseSet>, IntelloError> {
        Ok(None)
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        Ok(vec![])
    }
    async fn delete(&self, _: &str, _: &str) -> Result<bool, IntelloError> {
        Ok(false)
    }
}

/// Stub implementation of GameSetRepository<FillBlankSet> for tests
pub struct StubFillBlankRepository;

#[async_trait]
impl GameSetRepository<FillBlankSet> for StubFillBlankRepository {
    async fn insert(&self, set: &FillBlankSet) -> Result<FillBlankSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_id(&self, _: &str, _: &str) -> Result<Option<FillBlankSet>, IntelloError> {
        Ok(None)
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<FillBlankSet>, IntelloError> {
        Ok(vec![])
    }
    async fn delete(&self, _: &str, _: &str) -> Result<bool, IntelloError> {
        Ok(false)
    }
}


/// Create a complete stub repository bundle for tests.
///
/// Use this when you need a full IntelloRepositories bundle but don't care
/// about any specific repository behavior (all return empty/default values).
pub fn stub_intello_repositories() -> crate::services::IntelloRepositories {
    crate::services::IntelloRepositories {
        qcm_repo: Arc::new(JsonQcmRepository::new(TEST_QCMSET_FILE)),
        ai_qcm_repo: Arc::new(JsonQcmRepository::new(TEST_AI_QCMSET_FILE)),
        open_question_repo: Arc::new(JsonOpenQuestionRepository::new(TEST_OPEN_QUESTION_FILE)),
        flashcard_repo: Arc::new(JsonFlashcardRepository::new(TEST_FLASHCARD_FILE)),
        true_false_repo: Arc::new(StubTrueOrFalseRepository),
        keywords_repo: Arc::new(StubKeywordsRepository),
        order_phrase_repo: Arc::new(StubOrderPhraseRepository),
        fill_blank_repo: Arc::new(StubFillBlankRepository),
    }
}

/// Create a test IntelloService with all required dependencies.
///
/// Uses the builder pattern with stub repositories for games not under test.
pub fn create_test_service() -> IntelloService {
    IntelloService::builder()
        .with_repositories(stub_intello_repositories())
        .with_openrouter(Arc::new(OpenRouterService::with_google_key(String::new(), None)))
        .with_cache(Arc::new(OpenQuestionCache::new()))
        .build()
        .expect("Test IntelloService setup should not fail")
}

/// Get test user ID (using a fixed test user for unit tests)
pub fn get_test_user_id() -> String {
    "test-user-intello".to_string()
}

