//! Intello Service - Learning Games Application
//!
//! A learning application that allows users to create, manage, and take
//! QCM quizzes, flashcards, and other games.
//!
//! # Architecture
//! - Domain entities: `*_domain.rs` files
//! - Business logic: `*_service.rs` files
//! - Storage: `infrastructure/`

// Domain modules
mod app_domain;
pub mod ai_usage_domain;
pub mod course_domain;
pub mod custom_question_domain;
pub mod enums_domain;
pub mod error_domain;
pub mod fill_blank_domain;
pub mod flashcard_domain;
pub mod ids_domain;
pub mod keywords_domain;
pub mod open_question_domain;
pub mod order_phrase_domain;
pub mod qcm_question_domain;
pub mod qcm_set_domain;
pub mod study_session_domain;
pub mod true_false_domain;
pub mod types_domain;

// Other public modules
pub mod games_registry_domain;

pub use app_domain::IntelloApp;
pub use games_registry_domain::{GameInstance, AVAILABLE_GAMES};
pub use ids_domain::{OptionId, QuestionId, SetId};

// Re-export error types


// Re-export enums
pub use enums_domain::Level;

// Re-export DocumentType from custom_question
pub use custom_question_domain::{CustomQuestion, DocumentType};

// Re-export main set types
pub use fill_blank_domain::FillBlankSet;
pub use flashcard_domain::FlashcardSet;
pub use keywords_domain::KeywordSet;
pub use open_question_domain::OpenQuestionSet;
pub use order_phrase_domain::OrderPhraseSet;
pub use qcm_set_domain::QcmSet;
pub use true_false_domain::TrueOrFalseSet;

// Re-export QCM types
pub use qcm_question_domain::QcmQuestion;

// Re-export question/option types
pub use fill_blank_domain::{FillBlankOption, FillBlankQuestion};
pub use flashcard_domain::Flashcard;
pub use keywords_domain::{Keyword, KeywordQuestion};
pub use open_question_domain::OpenQuestion;
pub use order_phrase_domain::{OrderPhraseQuestion, OrderPhraseWord};
pub use true_false_domain::TrueOrFalseStatement;

// Service modules
mod ai_usage_service;
mod course_service;
pub mod course_parser_service;
pub mod learn_mode_domain;
pub mod learn_mode_service;
pub mod crud_service;
pub mod document_extractor_service;
mod fill_blank_service;
mod flashcard_service;
mod games_service;
mod keywords_service;
pub mod open_question_cache_service;
mod open_question_service;
mod order_phrase_service;
pub mod prompt_builder_service;
mod qcm_service;
mod study_session_service;
mod true_false_service;
mod validation_service;

pub use types_domain::{
    CheckAnswersInput, GenerateContentInput, IntelloRepositories,
    IntelloService, UserAnswer,
};
