//! IntelloService struct, constructor, and input/output types

use crate::services::intello::Level;
use crate::infra::{
    AiUsageRepository, CourseRepository, FillBlankRepository, FlashcardRepository,
    KeywordsRepository, OpenQuestionRepository, OrderPhraseRepository, QcmRepository,
    StudySessionRepository, TrueOrFalseRepository,
};
use crate::services::OpenRouterService;
use crate::services::intello::open_question_cache_service::OpenQuestionCache;
use std::fmt;
use std::sync::Arc;
use tracing::info;

// == INPUT TYPES FOR AI GENERATION ==

/// Input for generating AI content (QCM, Open Questions, Flashcards, or True/False)
#[derive(Debug, Clone)]
pub struct GenerateContentInput {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub language: String,
    pub level: Level,
    pub subjects: Vec<String>,
    pub num_questions: u8,
    /// Document contents: (filename, content, token_count)
    pub documents: Vec<(String, String, u32)>,
}

/// Input for checking/grading open question answers
#[derive(Debug, Clone)]
pub struct CheckAnswersInput {
    pub set_id: String,
    pub answers: Vec<UserAnswer>,
}

/// A user's answer to an open question
#[derive(Debug, Clone)]
pub struct UserAnswer {
    pub question_id: String,
    pub user_answer: String,
}

/// Result of grading answers
#[derive(Debug, Clone)]
pub struct GradingResult {
    pub question_id: String,
    pub grade: AnswerGrade,
    pub feedback: String,
}

/// Grade for a user's answer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnswerGrade {
    Right,
    Medium,
    Error,
}

// == REPOSITORY BUNDLE ==

/// Bundle of all Intello repositories for cleaner dependency injection.
///
/// Instead of passing 8+ repositories as individual parameters, group them
/// into this struct for easier construction and testing.
pub struct IntelloRepositories {
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

// == INTELLO SERVICE STRUCT ==

/// Service for managing all Intello operations (QCM, Open Questions, Flashcards, True/False, Keywords, Order Phrase, Fill Blank)
///
/// This service uses generic repositories that implement the respective traits,
/// allowing for different storage backends (JSON, database, etc.)
///
/// # Construction
///
/// Prefer using the builder pattern for new code:
/// ```ignore
/// let service = IntelloService::builder()
///     .with_repositories(repos)
///     .with_openrouter(openrouter_service)
///     .with_cache(cache)
///     .build()?;
/// ```
pub struct IntelloService {
    // QCM repositories
    pub(super) qcm_repo: Arc<dyn QcmRepository>,
    pub(super) ai_qcm_repo: Arc<dyn QcmRepository>,
    // Open Question repository
    pub(super) open_question_repo: Arc<dyn OpenQuestionRepository>,
    // Flashcard repository
    pub(super) flashcard_repo: Arc<dyn FlashcardRepository>,
    // True or False repository
    pub(super) true_false_repo: Arc<dyn TrueOrFalseRepository>,
    // Keywords repository
    pub(super) keywords_repo: Arc<dyn KeywordsRepository>,
    // Order Phrase repository
    pub(super) order_phrase_repo: Arc<dyn OrderPhraseRepository>,
    // Fill Blank repository
    pub(super) fill_blank_repo: Arc<dyn FillBlankRepository>,
    // Course Resource repository
    pub course_repo: Arc<dyn CourseRepository>,
    // AI Usage repository for cost tracking
    pub(super) ai_usage_repo: Arc<dyn AiUsageRepository>,
    // Study Session repository
    pub(super) study_session_repo: Arc<dyn StudySessionRepository>,
    // AI service for content generation
    pub openrouter_service: Arc<OpenRouterService>,
    // Cache for open question source content
    pub(super) open_question_cache: Arc<OpenQuestionCache>,
}

impl IntelloService {
    /// Create a new IntelloService from a repository bundle.
    pub fn from_repositories(
        repos: IntelloRepositories,
        openrouter_service: Arc<OpenRouterService>,
        open_question_cache: Arc<OpenQuestionCache>,
    ) -> Self {
        info!("IntelloService initialized with repository bundle and AI service");
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
            openrouter_service,
            open_question_cache,
        }
    }

    /// Create a builder for constructing IntelloService with fluent API.
    pub fn builder() -> IntelloServiceBuilder {
        IntelloServiceBuilder::new()
    }
}

// == INTELLO SERVICE BUILDER ==

/// Builder for IntelloService with fluent API.
///
/// # Example
/// ```ignore
/// let service = IntelloService::builder()
///     .with_repositories(repos)
///     .with_openrouter(openrouter_service)
///     .with_cache(cache)
///     .build()
///     .expect("Missing required dependencies");
/// ```
pub struct IntelloServiceBuilder {
    repos: Option<IntelloRepositories>,
    openrouter_service: Option<Arc<OpenRouterService>>,
    open_question_cache: Option<Arc<OpenQuestionCache>>,
}

impl IntelloServiceBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            repos: None,
            openrouter_service: None,
            open_question_cache: None,
        }
    }

    /// Set all repositories from a bundle.
    pub fn with_repositories(mut self, repos: IntelloRepositories) -> Self {
        self.repos = Some(repos);
        self
    }

    /// Set the OpenRouter AI service.
    pub fn with_openrouter(mut self, service: Arc<OpenRouterService>) -> Self {
        self.openrouter_service = Some(service);
        self
    }

    /// Set the open question cache.
    pub fn with_cache(mut self, cache: Arc<OpenQuestionCache>) -> Self {
        self.open_question_cache = Some(cache);
        self
    }

    /// Build the IntelloService.
    ///
    /// # Errors
    /// Returns an error string if any required dependency is missing.
    pub fn build(self) -> Result<IntelloService, &'static str> {
        let repos = self.repos.ok_or("Missing repositories")?;
        let openrouter_service = self
            .openrouter_service
            .ok_or("Missing OpenRouter service")?;
        let open_question_cache = self
            .open_question_cache
            .ok_or("Missing OpenQuestion cache")?;

        Ok(IntelloService::from_repositories(
            repos,
            openrouter_service,
            open_question_cache,
        ))
    }
}

impl Default for IntelloServiceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// == DEBUG AND DISPLAY IMPLEMENTATIONS ==

impl fmt::Debug for IntelloService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntelloService").finish()
    }
}

impl fmt::Display for IntelloService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntelloService")
    }
}
