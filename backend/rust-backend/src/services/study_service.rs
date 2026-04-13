//! StudyService struct, builder, and repository bundle
//!
//! Service-layer entry point for all Study operations.
//! Methods are spread across game-specific service files
//! via `impl StudyService` blocks.

use crate::infra::openrouter::OpenRouterClient;
use crate::infra::{
    AiUsageRepository, CourseRepository,
    FillBlankRepository, FlashcardRepository,
    KeywordsRepository, OpenQuestionRepository,
    OrderPhraseRepository, QcmRepository,
    StudySessionRepository, TrueOrFalseRepository,
};
use crate::services::games::open_question::open_question_cache_service::OpenQuestionCache;
use std::fmt;
use std::sync::Arc;
use tracing::info;

/// Bundle of all Study repositories for dependency
/// injection.
pub struct StudyRepositories {
    pub qcm_repo: Arc<dyn QcmRepository>,
    pub ai_qcm_repo: Arc<dyn QcmRepository>,
    pub open_question_repo: Arc<dyn OpenQuestionRepository>,
    pub flashcard_repo: Arc<dyn FlashcardRepository>,
    pub true_false_repo: Arc<dyn TrueOrFalseRepository>,
    pub keywords_repo: Arc<dyn KeywordsRepository>,
    pub order_phrase_repo: Arc<dyn OrderPhraseRepository>,
    pub fill_blank_repo: Arc<dyn FillBlankRepository>,
    pub course_repo: Arc<dyn CourseRepository>,
    pub ai_usage_repo: Arc<dyn AiUsageRepository>,
    pub study_session_repo: Arc<dyn StudySessionRepository>,
}

/// Service for managing all Study operations.
///
/// Methods are defined via `impl StudyService` blocks in
/// game-specific service files (qcm, flashcard, etc.).
///
/// # Construction
/// ```ignore
/// let service = StudyService::builder()
///     .with_repositories(repos)
///     .with_openrouter(client)
///     .with_cache(cache)
///     .build()?;
/// ```
pub struct StudyService {
    /// QCM repository
    pub(crate) qcm_repo: Arc<dyn QcmRepository>,
    /// AI-generated QCM repository
    pub(crate) ai_qcm_repo: Arc<dyn QcmRepository>,
    /// Open question repository
    pub(crate) open_question_repo:
        Arc<dyn OpenQuestionRepository>,
    /// Flashcard repository
    pub(crate) flashcard_repo:
        Arc<dyn FlashcardRepository>,
    /// True/false repository
    pub(crate) true_false_repo:
        Arc<dyn TrueOrFalseRepository>,
    /// Keywords repository
    pub(crate) keywords_repo:
        Arc<dyn KeywordsRepository>,
    /// Order phrase repository
    pub(crate) order_phrase_repo:
        Arc<dyn OrderPhraseRepository>,
    /// Fill blank repository
    pub(crate) fill_blank_repo:
        Arc<dyn FillBlankRepository>,
    /// Course repository
    pub course_repo: Arc<dyn CourseRepository>,
    /// AI usage repository for cost tracking
    pub(crate) ai_usage_repo:
        Arc<dyn AiUsageRepository>,
    /// Study session repository
    pub study_session_repo:
        Arc<dyn StudySessionRepository>,
    /// AI HTTP client for content generation
    pub(crate) openrouter_client:
        Arc<OpenRouterClient>,
    /// Cache for open question source content
    pub(crate) open_question_cache:
        Arc<OpenQuestionCache>,
}

impl StudyService {
    /// Create from a repository bundle.
    pub fn from_repositories(
        repos: StudyRepositories,
        openrouter_client: Arc<OpenRouterClient>,
        open_question_cache: Arc<OpenQuestionCache>,
    ) -> Self {
        info!("StudyService initialized");
        Self {
            qcm_repo: repos.qcm_repo,
            ai_qcm_repo: repos.ai_qcm_repo,
            open_question_repo: repos.open_question_repo,
            flashcard_repo: repos.flashcard_repo,
            true_false_repo: repos.true_false_repo,
            keywords_repo: repos.keywords_repo,
            order_phrase_repo: repos.order_phrase_repo,
            fill_blank_repo: repos.fill_blank_repo,
            course_repo: repos.course_repo,
            ai_usage_repo: repos.ai_usage_repo,
            study_session_repo: repos.study_session_repo,
            openrouter_client,
            open_question_cache,
        }
    }

    /// Create a builder for fluent construction.
    pub fn builder() -> StudyServiceBuilder {
        StudyServiceBuilder::new()
    }
}

impl fmt::Debug for StudyService {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("StudyService").finish()
    }
}

impl fmt::Display for StudyService {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "StudyService")
    }
}

/// Builder for StudyService with fluent API.
///
/// ```ignore
/// let service = StudyService::builder()
///     .with_repositories(repos)
///     .with_openrouter(client)
///     .with_cache(cache)
///     .build()?;
/// ```
pub struct StudyServiceBuilder {
    repos: Option<StudyRepositories>,
    openrouter_client: Option<Arc<OpenRouterClient>>,
    open_question_cache: Option<Arc<OpenQuestionCache>>,
}

impl StudyServiceBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            repos: None,
            openrouter_client: None,
            open_question_cache: None,
        }
    }

    /// Set all repositories from a bundle.
    pub fn with_repositories(
        mut self,
        repos: StudyRepositories,
    ) -> Self {
        self.repos = Some(repos);
        self
    }

    /// Set the OpenRouter HTTP client.
    pub fn with_openrouter(
        mut self,
        client: Arc<OpenRouterClient>,
    ) -> Self {
        self.openrouter_client = Some(client);
        self
    }

    /// Set the open question cache.
    pub fn with_cache(
        mut self,
        cache: Arc<OpenQuestionCache>,
    ) -> Self {
        self.open_question_cache = Some(cache);
        self
    }

    /// Build the StudyService.
    ///
    /// # Errors
    /// Returns an error if any dependency is missing.
    pub fn build(
        self,
    ) -> Result<StudyService, &'static str> {
        let repos =
            self.repos.ok_or("Missing repositories")?;
        let client = self
            .openrouter_client
            .ok_or("Missing OpenRouter client")?;
        let cache = self
            .open_question_cache
            .ok_or("Missing OpenQuestion cache")?;
        Ok(StudyService::from_repositories(
            repos, client, cache,
        ))
    }
}

impl Default for StudyServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_new_has_no_fields_set() {
        let builder = StudyServiceBuilder::new();
        assert!(builder.repos.is_none());
        assert!(builder.openrouter_client.is_none());
        assert!(builder.open_question_cache.is_none());
    }

    #[test]
    fn test_builder_default_same_as_new() {
        let builder = StudyServiceBuilder::default();
        assert!(builder.repos.is_none());
        assert!(builder.openrouter_client.is_none());
        assert!(builder.open_question_cache.is_none());
    }

    #[test]
    fn test_builder_build_without_repos_returns_error() {
        let builder = StudyServiceBuilder::new();
        let result = builder.build();
        assert_eq!(
            result.unwrap_err(),
            "Missing repositories"
        );
    }

    #[test]
    fn test_service_builder_returns_builder() {
        let builder = StudyService::builder();
        assert!(builder.repos.is_none());
    }
}
