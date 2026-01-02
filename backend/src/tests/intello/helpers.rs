//! Intello test helpers

use crate::services::intello::ai_usage_domain::{AiUsageLog, CreateAiUsageLog};
use crate::services::intello::{
    fill_blank_domain::FillBlankSet, flashcard_domain::FlashcardSet, keywords_domain::KeywordSet, 
    open_question_domain::OpenQuestionSet, order_phrase_domain::OrderPhraseSet, 
    qcm_set_domain::QcmSet, true_false_domain::TrueOrFalseSet,
};
use crate::shared::AppError;
use crate::services::intello::types_domain::IntelloService;
use crate::services::intello::error_domain::IntelloError;
use crate::infra::{AiUsageRepository, GameSetRepository, OpenQuestionRepository, QcmRepository};
use crate::services::{OpenRouterService};
use crate::services::intello::open_question_cache_service::OpenQuestionCache;
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
        Self {
            sets: Mutex::new(vec![]),
        }
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
    async fn find_by_id(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<QcmSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets
            .iter()
            .find(|s| s.id.as_str() == set_id && s.user_id.as_str() == user_id)
            .cloned())
    }
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<QcmSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets
            .iter()
            .filter(|s| s.user_id.as_str() == user_id)
            .cloned()
            .collect())
    }
    async fn update(&self, set: &QcmSet) -> Result<bool, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        if let Some(existing) = sets
            .iter_mut()
            .find(|s| s.id == set.id && s.user_id == set.user_id)
        {
            *existing = set.clone();
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        let len_before = sets.len();
        sets.retain(|s| !(s.id.as_str() == set_id && s.user_id.as_str() == user_id));
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
        Self {
            sets: Mutex::new(vec![]),
        }
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
    async fn find_by_id(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<OpenQuestionSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets
            .iter()
            .find(|s| s.id.as_str() == set_id && s.user_id.as_str() == user_id)
            .cloned())
    }
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets
            .iter()
            .filter(|s| s.user_id.as_str() == user_id)
            .cloned()
            .collect())
    }
}

// == STUB FLASHCARD REPOSITORY ==

/// Stub implementation of GameSetRepository<FlashcardSet> for tests
pub struct StubFlashcardRepository {
    sets: Mutex<Vec<FlashcardSet>>,
}

impl StubFlashcardRepository {
    pub fn new() -> Self {
        Self {
            sets: Mutex::new(vec![]),
        }
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
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<FlashcardSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets
            .iter()
            .filter(|s| s.user_id.as_str() == user_id)
            .cloned()
            .collect())
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
    async fn find_by_user(&self, _: &str) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        Ok(vec![])
    }
}

/// Stub implementation of GameSetRepository<KeywordSet> for tests
pub struct StubKeywordsRepository;

#[async_trait]
impl GameSetRepository<KeywordSet> for StubKeywordsRepository {
    async fn insert(&self, set: &KeywordSet) -> Result<KeywordSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<KeywordSet>, IntelloError> {
        Ok(vec![])
    }
}

/// Stub implementation of GameSetRepository<OrderPhraseSet> for tests
pub struct StubOrderPhraseRepository;

#[async_trait]
impl GameSetRepository<OrderPhraseSet> for StubOrderPhraseRepository {
    async fn insert(&self, set: &OrderPhraseSet) -> Result<OrderPhraseSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        Ok(vec![])
    }
}

/// Stub implementation of GameSetRepository<FillBlankSet> for tests
pub struct StubFillBlankRepository;

#[async_trait]
impl GameSetRepository<FillBlankSet> for StubFillBlankRepository {
    async fn insert(&self, set: &FillBlankSet) -> Result<FillBlankSet, IntelloError> {
        Ok(set.clone())
    }
    async fn find_by_user(&self, _: &str) -> Result<Vec<FillBlankSet>, IntelloError> {
        Ok(vec![])
    }
}

// == STUB COURSE REPOSITORY ==

pub struct StubCourseRepository;

#[async_trait]
impl crate::infra::CourseRepository for StubCourseRepository {
    async fn create_course(
        &self,
        course: &crate::services::intello::course_domain::Course,
    ) -> Result<crate::services::intello::course_domain::Course, AppError> {
        Ok(course.clone())
    }
    async fn get_user_courses(
        &self,
        _: &str,
    ) -> Result<Vec<crate::services::intello::course_domain::Course>, AppError> {
        Ok(vec![])
    }
    async fn get_course_resources(
        &self,
        _: &str,
    ) -> Result<Vec<crate::services::intello::course_domain::UserResource>, AppError> {
        Ok(vec![])
    }
    async fn fetch_resources(
        &self,
        _: &[String],
    ) -> Result<Vec<crate::services::intello::course_domain::UserResource>, AppError> {
        Ok(vec![])
    }
    async fn get_user_resources(
        &self,
        _: &str,
    ) -> Result<Vec<crate::services::intello::course_domain::ResourceSummary>, AppError> {
        Ok(vec![])
    }
    async fn get_resource_by_id(
        &self,
        _: &str,
    ) -> Result<Option<crate::services::intello::course_domain::UserResource>, AppError> {
        Ok(None)
    }
    async fn resource_exists(&self, _: &str, _: &str) -> Result<bool, AppError> {
        Ok(false)
    }
    async fn create_user_resource(
        &self,
        resource: &crate::services::intello::course_domain::UserResource,
    ) -> Result<crate::services::intello::course_domain::UserResource, AppError> {
        Ok(resource.clone())
    }
    async fn link_resource_to_course(&self, _: &str, _: &str) -> Result<(), AppError> {
        Ok(())
    }
}

// == STUB AI USAGE REPOSITORY ==

pub struct StubAiUsageRepository;

#[async_trait]
impl AiUsageRepository for StubAiUsageRepository {
    async fn log_usage(&self, _input: CreateAiUsageLog) -> Result<AiUsageLog, AppError> {
        Ok(AiUsageLog {
            id: "test-id".to_string(),
            user_id: "test-user".to_string(),
            feature_type: "test".to_string(),
            model_id: "test-model".to_string(),
            input_tokens: 0,
            output_tokens: 0,
            input_cost_usd: 0.0,
            output_cost_usd: 0.0,
            total_cost_usd: 0.0,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }
    async fn get_user_usage(&self, _user_id: &str) -> Result<Vec<AiUsageLog>, AppError> {
        Ok(vec![])
    }
    async fn get_all_usage(&self) -> Result<Vec<AiUsageLog>, AppError> {
        Ok(vec![])
    }
}

// == STUB STUDY SESSION REPOSITORY ==

pub struct StubStudySessionRepository;

#[async_trait]
impl crate::infra::StudySessionRepository for StubStudySessionRepository {
    async fn create(
        &self,
        session: &crate::services::intello::study_session_domain::StudySession,
    ) -> Result<crate::services::intello::study_session_domain::StudySession, AppError> {
        Ok(session.clone())
    }

    async fn list_by_course(
        &self,
        _course_id: &str,
    ) -> Result<Vec<crate::services::intello::study_session_domain::StudySession>, AppError> {
        Ok(vec![])
    }

    async fn get(
        &self,
        _session_id: &str,
    ) -> Result<crate::services::intello::study_session_domain::StudySession, AppError> {
        Err(AppError::Internal(crate::http_api::utils::InternalError::new("Not implemented in stub".to_string())))
    }

    async fn save_session_content(
        &self,
        _session_id: &str,
        _content: &serde_json::Value,
    ) -> Result<(), AppError> {
        Ok(())
    }
}

// == HELPER FUNCTIONS ==

/// Create a complete stub repository bundle for tests.
#[allow(dead_code)]
pub fn stub_intello_repositories() -> crate::services::intello::types_domain::IntelloRepositories {
    crate::services::intello::types_domain::IntelloRepositories {
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
    }
}

/// Create a test IntelloService with all required dependencies.
#[allow(dead_code)]
pub fn create_test_service() -> IntelloService {
    IntelloService::builder()
        .with_repositories(stub_intello_repositories())
        .with_openrouter(Arc::new(crate::infra::openrouter::OpenRouterClient::with_google_key(
            String::new(),
            None,
        )))
        .with_cache(Arc::new(OpenQuestionCache::new()))
        .build()
        .expect("Test IntelloService setup should not fail")
}

/// Get test user ID (using a fixed test user for unit tests)
#[allow(dead_code)]
pub fn get_test_user_id() -> String {
    "test-user-intello".to_string()
}
