//! Intello test helpers

use crate::domain::intello::{FillBlankSet, FlashcardSet, KeywordSet, OpenQuestionSet, OrderPhraseSet, QcmSet, TrueOrFalseSet};
use crate::error::{AppError, IntelloError};
use crate::infrastructure::{GameSetRepository, OpenQuestionRepository, QcmRepository};
use crate::services::{IntelloService, OpenRouterService};
use crate::shared::OpenQuestionCache;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

// == STUB QCM REPOSITORY ==

/// Stub implementation of QcmRepository for tests
/// Stores data in memory using a Mutex for thread safety
pub struct StubQcmRepository {
    sets: Mutex<Vec<QcmSet>>,
}

impl StubQcmRepository {
    pub fn new() -> Self {
        Self { sets: Mutex::new(vec![]) }
    }
}

impl Default for StubQcmRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl QcmRepository for StubQcmRepository {
    async fn insert(&self, set: &QcmSet) -> Result<QcmSet, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        sets.push(set.clone());
        Ok(set.clone())
    }
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<QcmSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets.iter().find(|s| s.id == set_id && s.user_id == user_id).cloned())
    }
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<QcmSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets.iter().filter(|s| s.user_id == user_id).cloned().collect())
    }
    async fn update(&self, set: &QcmSet) -> Result<bool, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        if let Some(existing) = sets.iter_mut().find(|s| s.id == set.id && s.user_id == set.user_id) {
            *existing = set.clone();
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        let len_before = sets.len();
        sets.retain(|s| !(s.id == set_id && s.user_id == user_id));
        Ok(sets.len() < len_before)
    }
}

// == STUB OPEN QUESTION REPOSITORY ==

/// Stub implementation of OpenQuestionRepository for tests
pub struct StubOpenQuestionRepository {
    sets: Mutex<Vec<OpenQuestionSet>>,
}

impl StubOpenQuestionRepository {
    pub fn new() -> Self {
        Self { sets: Mutex::new(vec![]) }
    }
}

impl Default for StubOpenQuestionRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl OpenQuestionRepository for StubOpenQuestionRepository {
    async fn insert(&self, set: &OpenQuestionSet) -> Result<OpenQuestionSet, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        sets.push(set.clone());
        Ok(set.clone())
    }
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<OpenQuestionSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets.iter().find(|s| s.id == set_id && s.user_id == user_id).cloned())
    }
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets.iter().filter(|s| s.user_id == user_id).cloned().collect())
    }
}

// == STUB FLASHCARD REPOSITORY ==

/// Stub implementation of GameSetRepository<FlashcardSet> for tests
pub struct StubFlashcardRepository {
    sets: Mutex<Vec<FlashcardSet>>,
}

impl StubFlashcardRepository {
    pub fn new() -> Self {
        Self { sets: Mutex::new(vec![]) }
    }
}

impl Default for StubFlashcardRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GameSetRepository<FlashcardSet> for StubFlashcardRepository {
    async fn insert(&self, set: &FlashcardSet) -> Result<FlashcardSet, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        sets.push(set.clone());
        Ok(set.clone())
    }
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<FlashcardSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets.iter().find(|s| s.id == set_id && s.user_id == user_id).cloned())
    }
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<FlashcardSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets.iter().filter(|s| s.user_id == user_id).cloned().collect())
    }
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        let len_before = sets.len();
        sets.retain(|s| !(s.id == set_id && s.user_id == user_id));
        Ok(sets.len() < len_before)
    }
}

// == STUB REPOSITORIES FOR OTHER GAME TYPES ==

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

// == STUB COURSE REPOSITORY ==

pub struct StubCourseRepository;

#[async_trait]
impl crate::infrastructure::CourseRepository for StubCourseRepository {
    async fn create_course(&self, course: &crate::domain::intello::course::Course) -> Result<crate::domain::intello::course::Course, AppError> {
        Ok(course.clone())
    }
    async fn get_user_courses(&self, _: &str) -> Result<Vec<crate::domain::intello::course::Course>, AppError> {
        Ok(vec![])
    }
    async fn add_resource(&self, resource: &crate::domain::intello::course::Resource) -> Result<crate::domain::intello::course::Resource, AppError> {
        Ok(resource.clone())
    }
    async fn get_course_resources(&self, _: &str) -> Result<Vec<crate::domain::intello::course::Resource>, AppError> {
        Ok(vec![])
    }
    async fn fetch_resources(&self, _: &[String]) -> Result<Vec<crate::domain::intello::course::Resource>, AppError> {
        Ok(vec![])
    }
}

// == HELPER FUNCTIONS ==

/// Create a complete stub repository bundle for tests.
pub fn stub_intello_repositories() -> crate::services::IntelloRepositories {
    crate::services::IntelloRepositories {
        qcm_repo: Arc::new(StubQcmRepository::new()),
        ai_qcm_repo: Arc::new(StubQcmRepository::new()),
        open_question_repo: Arc::new(StubOpenQuestionRepository::new()),
        flashcard_repo: Arc::new(StubFlashcardRepository::new()),
        true_false_repo: Arc::new(StubTrueOrFalseRepository),
        keywords_repo: Arc::new(StubKeywordsRepository),
        order_phrase_repo: Arc::new(StubOrderPhraseRepository),
        fill_blank_repo: Arc::new(StubFillBlankRepository),
        course_repo: Arc::new(StubCourseRepository),
    }
}

/// Create a test IntelloService with all required dependencies.
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
