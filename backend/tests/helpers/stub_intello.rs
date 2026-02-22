//! Stub repositories and factory for IntelloService integration tests

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use LAPP::infra::openrouter::OpenRouterClient;
use LAPP::infra::{
    AiUsageRepository, CourseRepository, GameSetRepository,
    OpenQuestionRepository, QcmRepository,
};
use LAPP::services::intello::ai_usage::ai_usage_domain::{
    AiUsageLog, CreateAiUsageLog,
};
use LAPP::services::intello::course::domain::{
    Course, ResourceSummary, UserResource,
};
use LAPP::services::intello::error_domain::IntelloError;
use LAPP::services::intello::games::open_question::open_question_cache_service::OpenQuestionCache;
use LAPP::services::intello::{
    FillBlankSet, FlashcardSet, IntelloRepositories, IntelloService,
    KeywordSet, OpenQuestionSet, OrderPhraseSet, QcmSet,
    TrueOrFalseSet,
};
use LAPP::shared::AppError;

// == STUB QCM REPOSITORY ==

/// In-memory QCM repository stub
pub struct StubQcmRepo {
    sets: Mutex<Vec<QcmSet>>,
}

impl StubQcmRepo {
    pub fn new() -> Self {
        Self {
            sets: Mutex::new(vec![]),
        }
    }
}

#[async_trait]
impl QcmRepository for StubQcmRepo {
    async fn insert(
        &self,
        set: &QcmSet,
    ) -> Result<QcmSet, IntelloError> {
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
            .find(|s| {
                s.id.as_str() == set_id
                    && s.user_id.as_str() == user_id
            })
            .cloned())
    }

    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<QcmSet>, IntelloError> {
        let sets = self.sets.lock().unwrap();
        Ok(sets
            .iter()
            .filter(|s| s.user_id.as_str() == user_id)
            .cloned()
            .collect())
    }

    async fn update(
        &self,
        set: &QcmSet,
    ) -> Result<bool, IntelloError> {
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

    async fn delete(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<bool, IntelloError> {
        let mut sets = self.sets.lock().unwrap();
        let before = sets.len();
        sets.retain(|s| {
            !(s.id.as_str() == set_id
                && s.user_id.as_str() == user_id)
        });
        Ok(sets.len() < before)
    }
}

// == NO-OP STUBS FOR OTHER REPOS ==

struct NoopOpenQuestion;
#[async_trait]
impl OpenQuestionRepository for NoopOpenQuestion {
    async fn insert(
        &self,
        s: &OpenQuestionSet,
    ) -> Result<OpenQuestionSet, IntelloError> {
        Ok(s.clone())
    }
    async fn find_by_id(
        &self,
        _: &str,
        _: &str,
    ) -> Result<Option<OpenQuestionSet>, IntelloError> {
        Ok(None)
    }
    async fn find_by_user(
        &self,
        _: &str,
    ) -> Result<Vec<OpenQuestionSet>, IntelloError> {
        Ok(vec![])
    }
}

struct NoopFlashcard;
#[async_trait]
impl GameSetRepository<FlashcardSet> for NoopFlashcard {
    async fn insert(
        &self,
        s: &FlashcardSet,
    ) -> Result<FlashcardSet, IntelloError> {
        Ok(s.clone())
    }
    async fn find_by_user(
        &self,
        _: &str,
    ) -> Result<Vec<FlashcardSet>, IntelloError> {
        Ok(vec![])
    }
}

struct NoopTrueFalse;
#[async_trait]
impl GameSetRepository<TrueOrFalseSet> for NoopTrueFalse {
    async fn insert(
        &self,
        s: &TrueOrFalseSet,
    ) -> Result<TrueOrFalseSet, IntelloError> {
        Ok(s.clone())
    }
    async fn find_by_user(
        &self,
        _: &str,
    ) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        Ok(vec![])
    }
}

struct NoopKeywords;
#[async_trait]
impl GameSetRepository<KeywordSet> for NoopKeywords {
    async fn insert(
        &self,
        s: &KeywordSet,
    ) -> Result<KeywordSet, IntelloError> {
        Ok(s.clone())
    }
    async fn find_by_user(
        &self,
        _: &str,
    ) -> Result<Vec<KeywordSet>, IntelloError> {
        Ok(vec![])
    }
}

struct NoopOrderPhrase;
#[async_trait]
impl GameSetRepository<OrderPhraseSet> for NoopOrderPhrase {
    async fn insert(
        &self,
        s: &OrderPhraseSet,
    ) -> Result<OrderPhraseSet, IntelloError> {
        Ok(s.clone())
    }
    async fn find_by_user(
        &self,
        _: &str,
    ) -> Result<Vec<OrderPhraseSet>, IntelloError> {
        Ok(vec![])
    }
}

struct NoopFillBlank;
#[async_trait]
impl GameSetRepository<FillBlankSet> for NoopFillBlank {
    async fn insert(
        &self,
        s: &FillBlankSet,
    ) -> Result<FillBlankSet, IntelloError> {
        Ok(s.clone())
    }
    async fn find_by_user(
        &self,
        _: &str,
    ) -> Result<Vec<FillBlankSet>, IntelloError> {
        Ok(vec![])
    }
}

struct NoopCourse;
#[async_trait]
impl CourseRepository for NoopCourse {
    async fn create_course(
        &self,
        c: &Course,
    ) -> Result<Course, AppError> {
        Ok(c.clone())
    }
    async fn get_user_courses(
        &self,
        _: &str,
    ) -> Result<Vec<Course>, AppError> {
        Ok(vec![])
    }
    async fn get_course_resources(
        &self,
        _: &str,
    ) -> Result<Vec<UserResource>, AppError> {
        Ok(vec![])
    }
    async fn fetch_resources(
        &self,
        _: &[String],
    ) -> Result<Vec<UserResource>, AppError> {
        Ok(vec![])
    }
    async fn get_user_resources(
        &self,
        _: &str,
    ) -> Result<Vec<ResourceSummary>, AppError> {
        Ok(vec![])
    }
    async fn get_resource_by_id(
        &self,
        _: &str,
    ) -> Result<Option<UserResource>, AppError> {
        Ok(None)
    }
    async fn resource_exists(
        &self,
        _: &str,
        _: &str,
    ) -> Result<bool, AppError> {
        Ok(false)
    }
    async fn create_user_resource(
        &self,
        r: &UserResource,
    ) -> Result<UserResource, AppError> {
        Ok(r.clone())
    }
    async fn link_resource_to_course(
        &self,
        _: &str,
        _: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
    async fn delete_course(
        &self,
        _: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
    async fn delete_resource_links(
        &self,
        _: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
    async fn delete_resource(
        &self,
        _: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
}

struct NoopAiUsage;
#[async_trait]
impl AiUsageRepository for NoopAiUsage {
    async fn log_usage(
        &self,
        _: CreateAiUsageLog,
    ) -> Result<AiUsageLog, AppError> {
        Ok(AiUsageLog {
            id: "stub".to_string(),
            user_id: "stub".to_string(),
            feature_type: "stub".to_string(),
            model_id: "stub".to_string(),
            input_tokens: 0,
            output_tokens: 0,
            input_cost_usd: 0.0,
            output_cost_usd: 0.0,
            total_cost_usd: 0.0,
            created_at: chrono::Utc::now().to_rfc3339(),
        })
    }
    async fn get_user_usage(
        &self,
        _: &str,
    ) -> Result<Vec<AiUsageLog>, AppError> {
        Ok(vec![])
    }
    async fn get_all_usage(
        &self,
    ) -> Result<Vec<AiUsageLog>, AppError> {
        Ok(vec![])
    }
}

struct NoopStudySession;
#[async_trait]
impl LAPP::infra::StudySessionRepository for NoopStudySession {
    async fn create(
        &self,
        s: &LAPP::services::intello::study_session::study_session_domain::StudySession,
    ) -> Result<
        LAPP::services::intello::study_session::study_session_domain::StudySession,
        AppError,
    > {
        Ok(s.clone())
    }
    async fn list_by_course(
        &self,
        _: &str,
    ) -> Result<
        Vec<LAPP::services::intello::study_session::study_session_domain::StudySession>,
        AppError,
    > {
        Ok(vec![])
    }
    async fn get(
        &self,
        _: &str,
    ) -> Result<
        LAPP::services::intello::study_session::study_session_domain::StudySession,
        AppError,
    > {
        Err(AppError::Internal(
            LAPP::http_api::utils::InternalError::new(
                "not implemented".to_string(),
            ),
        ))
    }
    async fn save_session_content(
        &self,
        _: &str,
        _: &serde_json::Value,
    ) -> Result<(), AppError> {
        Ok(())
    }
    async fn delete_by_course(
        &self,
        _: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
    async fn delete(
        &self,
        _: &str,
    ) -> Result<(), AppError> {
        Ok(())
    }
}

/// Build an IntelloService with in-memory stubs
pub fn make_intello_service() -> IntelloService {
    let repos = IntelloRepositories {
        qcm_repo: Arc::new(StubQcmRepo::new()),
        ai_qcm_repo: Arc::new(StubQcmRepo::new()),
        open_question_repo: Arc::new(NoopOpenQuestion),
        flashcard_repo: Arc::new(NoopFlashcard),
        true_false_repo: Arc::new(NoopTrueFalse),
        keywords_repo: Arc::new(NoopKeywords),
        order_phrase_repo: Arc::new(NoopOrderPhrase),
        fill_blank_repo: Arc::new(NoopFillBlank),
        course_repo: Arc::new(NoopCourse),
        ai_usage_repo: Arc::new(NoopAiUsage),
        study_session_repo: Arc::new(NoopStudySession),
    };
    IntelloService::builder()
        .with_repositories(repos)
        .with_openrouter(Arc::new(
            OpenRouterClient::with_google_key(String::new(), None),
        ))
        .with_cache(Arc::new(OpenQuestionCache::new()))
        .build()
        .expect("test IntelloService setup")
}
