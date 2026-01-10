//! IntelloService struct, constructor, and input/output types

use crate::infra::openrouter::OpenRouterClient;
use crate::infra::{
    AiUsageRepository, CourseRepository, FillBlankRepository, FlashcardRepository,
    KeywordsRepository, OpenQuestionRepository, OrderPhraseRepository, QcmRepository,
    StudySessionRepository, TrueOrFalseRepository,
};
use crate::services::intello::games::open_question::open_question_cache_service::OpenQuestionCache;
use crate::services::intello::Level;
use std::fmt;
use std::sync::Arc;
use tracing::info;

// == INPUT TYPES FOR AI GENERATION ==

// ** GenerateContentInput **
// ==> Input parameters for AI content generation (QCM, Flashcards, etc.)
//
// @ name : Name of the generated set
// @ description : Description of the set's purpose
// @ instructions : Specific instructions for the AI model
// @ language : Language code (e.g., "en", "fr")
// @ level : Difficulty level (Easy, Medium, Hard)
// @ subjects : Subject tags (max 3)
// @ num_questions : Number of questions to generate
// @ documents : Uploaded documents as (filename, content, token_count) tuples
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateContentInput {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub language: String,
    pub level: Level,
    pub subjects: Vec<String>,
    pub num_questions: u8,
    pub documents: Vec<(String, String, u32)>,
}

// ** CheckAnswersInput **
// ==> Input for checking/grading open question answers
//
// @ set_id : The question set ID
// @ answers : User's answers to be graded
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckAnswersInput {
    pub set_id: String,
    pub answers: Vec<UserAnswer>,
}

// ** UserAnswer **
// ==> A user's answer to a single open question
//
// @ question_id : ID of the question being answered
// @ user_answer : User's written answer text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAnswer {
    pub question_id: String,
    pub user_answer: String,
}

// ** GradingResult **
// ==> Result of grading a single answer
//
// @ question_id : ID of the graded question
// @ grade : Grade assigned (Right, Medium, Error)
// @ feedback : AI-generated feedback for the answer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradingResult {
    pub question_id: String,
    pub grade: AnswerGrade,
    pub feedback: String,
}

// ** AnswerGrade **
// ==> Grade categories for user answers
//
// @ Right : Answer is correct
// @ Medium : Answer is partially correct
// @ Error : Answer is incorrect
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
    pub(crate) qcm_repo: Arc<dyn QcmRepository>,
    pub(crate) ai_qcm_repo: Arc<dyn QcmRepository>,
    // Open Question repository
    pub(crate) open_question_repo: Arc<dyn OpenQuestionRepository>,
    // Flashcard repository
    pub(crate) flashcard_repo: Arc<dyn FlashcardRepository>,
    // True or False repository
    pub(crate) true_false_repo: Arc<dyn TrueOrFalseRepository>,
    // Keywords repository
    pub(crate) keywords_repo: Arc<dyn KeywordsRepository>,
    // Order Phrase repository
    pub(crate) order_phrase_repo: Arc<dyn OrderPhraseRepository>,
    // Fill Blank repository
    pub(crate) fill_blank_repo: Arc<dyn FillBlankRepository>,
    // Course Resource repository
    pub course_repo: Arc<dyn CourseRepository>,
    // AI Usage repository for cost tracking
    pub(crate) ai_usage_repo: Arc<dyn AiUsageRepository>,
    // Study Session repository
    pub study_session_repo: Arc<dyn StudySessionRepository>,
    // AI HTTP client for content generation
    pub(crate) openrouter_client: Arc<OpenRouterClient>,
    // Cache for open question source content
    pub(crate) open_question_cache: Arc<OpenQuestionCache>,
}

impl IntelloService {
    /// Create a new IntelloService from a repository bundle.
    pub fn from_repositories(
        repos: IntelloRepositories,
        openrouter_client: Arc<OpenRouterClient>,
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
            openrouter_client,
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
    openrouter_client: Option<Arc<OpenRouterClient>>,
    open_question_cache: Option<Arc<OpenQuestionCache>>,
}

impl IntelloServiceBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            repos: None,
            openrouter_client: None,
            open_question_cache: None,
        }
    }

    /// Set all repositories from a bundle.
    pub fn with_repositories(mut self, repos: IntelloRepositories) -> Self {
        self.repos = Some(repos);
        self
    }

    /// Set the OpenRouter HTTP client.
    pub fn with_openrouter(mut self, client: Arc<OpenRouterClient>) -> Self {
        self.openrouter_client = Some(client);
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
        let openrouter_client = self.openrouter_client.ok_or("Missing OpenRouter client")?;
        let open_question_cache = self
            .open_question_cache
            .ok_or("Missing OpenQuestion cache")?;

        Ok(IntelloService::from_repositories(
            repos,
            openrouter_client,
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
