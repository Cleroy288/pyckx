//! IntelloService struct, constructor, and input/output types

use crate::infrastructure::{FlashcardRepository, KeywordsRepository, OpenQuestionRepository, OrderPhraseRepository, QcmRepository, TrueOrFalseRepository};
use crate::services::OpenRouterService;
use crate::shared::OpenQuestionCache;
use crate::domain::intello::Level;
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
    /// AI model to use (None = default)
    pub model: Option<String>,
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

// == INTELLO SERVICE STRUCT ==

/// Service for managing all Intello operations (QCM, Open Questions, Flashcards, True/False, Keywords, Order Phrase)
///
/// This service uses generic repositories that implement the respective traits,
/// allowing for different storage backends (JSON, database, etc.)
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
    // AI service for content generation
    pub(super) openrouter_service: Arc<OpenRouterService>,
    // Cache for open question source content
    pub(super) open_question_cache: Arc<OpenQuestionCache>,
}

impl IntelloService {
    /// Create a new IntelloService with all required dependencies
    pub fn new(
        qcm_repo: Arc<dyn QcmRepository>,
        ai_qcm_repo: Arc<dyn QcmRepository>,
        open_question_repo: Arc<dyn OpenQuestionRepository>,
        flashcard_repo: Arc<dyn FlashcardRepository>,
        true_false_repo: Arc<dyn TrueOrFalseRepository>,
        keywords_repo: Arc<dyn KeywordsRepository>,
        order_phrase_repo: Arc<dyn OrderPhraseRepository>,
        openrouter_service: Arc<OpenRouterService>,
        open_question_cache: Arc<OpenQuestionCache>,
    ) -> Self {
        info!("IntelloService initialized with all repositories and AI service");
        Self {
            qcm_repo,
            ai_qcm_repo,
            open_question_repo,
            flashcard_repo,
            true_false_repo,
            keywords_repo,
            order_phrase_repo,
            openrouter_service,
            open_question_cache,
        }
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

