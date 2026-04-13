//! Shared test stubs for Study service unit tests

mod course_stub;
mod factories;
mod session_stub;

pub use course_stub::MemCourseRepo;
pub use factories::{
    make_course, make_resource, make_session,
};
pub use session_stub::MemSessionRepo;

use crate::infra::{
    CourseRepository, StudySessionRepository,
};
use std::sync::Arc;

/// Build StudyService with custom course + session
/// repos. All other repos use no-op stubs.
pub fn build_test_service(
    course_repo: Arc<dyn CourseRepository>,
    session_repo: Arc<dyn StudySessionRepository>,
) -> super::StudyService {
    use crate::infra::openrouter::OpenRouterClient;
    use crate::services::games::open_question::open_question_cache_service::OpenQuestionCache;
    use crate::tests::helpers::{
        StubAiUsageRepository,
        StubFillBlankRepository,
        StubFlashcardRepository,
        StubKeywordsRepository,
        StubOpenQuestionRepository,
        StubOrderPhraseRepository,
        StubQcmRepository,
        StubTrueOrFalseRepository,
    };

    let repos = super::StudyRepositories {
        qcm_repo: Arc::new(StubQcmRepository::new()),
        ai_qcm_repo: Arc::new(
            StubQcmRepository::new(),
        ),
        open_question_repo: Arc::new(
            StubOpenQuestionRepository::new(),
        ),
        flashcard_repo: Arc::new(
            StubFlashcardRepository::new(),
        ),
        true_false_repo: Arc::new(
            StubTrueOrFalseRepository,
        ),
        keywords_repo: Arc::new(
            StubKeywordsRepository,
        ),
        order_phrase_repo: Arc::new(
            StubOrderPhraseRepository,
        ),
        fill_blank_repo: Arc::new(
            StubFillBlankRepository,
        ),
        course_repo,
        ai_usage_repo: Arc::new(StubAiUsageRepository),
        study_session_repo: session_repo,
    };

    super::StudyService::builder()
        .with_repositories(repos)
        .with_openrouter(Arc::new(
            OpenRouterClient::with_google_key(
                String::new(),
                None,
            ),
        ))
        .with_cache(Arc::new(OpenQuestionCache::new()))
        .build()
        .expect("Test service build should not fail")
}
