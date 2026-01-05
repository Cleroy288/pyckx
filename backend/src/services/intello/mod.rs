//! Intello Service - Learning Games Application
//!
//! A learning application that allows users to create, manage, and take
//! QCM quizzes, flashcards, and other games.
//!
//! # Architecture
//! - Domain entities: `games/*/domain.rs` files
//! - Business logic: `games/*/service.rs` files
//! - Storage: `infrastructure/`

// Domain modules
pub mod ai_response; // AI response schemas from OpenRouter
pub mod ai_usage;
pub mod document_extractor;
pub mod app;
pub mod course; // Course module with domain, parser, prompt, and service
pub mod domain;
pub mod learn_mode;
pub mod games; // New modular games structure
pub mod tracking;
pub mod utils;

// Backward-compatible re-exports for domain submodules
// Allows old import paths like `intello::error_domain::*` to work
pub use domain::error_domain;
pub use domain::types_domain;
pub use domain::custom_question_domain;

// Backward-compatible re-exports for relocated services/modules

// Additional type re-exports for backward compatibility


// Other public modules

pub use app::app_domain::IntelloApp;
pub use app::games_registry_domain::{GameInstance, AVAILABLE_GAMES};
pub use domain::ids_domain::{OptionId, QuestionId, SetId};

// Re-export error types

// Re-export enums
pub use domain::enums_domain::Level;

// Re-export DocumentType from custom_question
pub use domain::custom_question_domain::{CustomQuestion, DocumentType};


// Re-export main set types from games module
pub use games::fill_blank::FillBlankSet;
pub use games::flashcard::FlashcardSet;
pub use games::keywords::KeywordSet;
pub use games::open_question::OpenQuestionSet;
pub use games::order_phrase::OrderPhraseSet;
pub use games::qcm::QcmSet;
pub use games::true_false::TrueOrFalseSet;

// Re-export QCM types
pub use games::qcm::QcmQuestion;

// Re-export question/option types
pub use games::fill_blank::{FillBlankOption, FillBlankQuestion};
pub use games::flashcard::Flashcard;
pub use games::keywords::{Keyword, KeywordQuestion};
pub use games::open_question::OpenQuestion;
pub use games::order_phrase::{OrderPhraseQuestion, OrderPhraseWord};
pub use games::true_false::TrueOrFalseStatement;


// Service modules

pub mod crud;

pub mod study_session;

pub use domain::types_domain::{
    CheckAnswersInput, GenerateContentInput, IntelloRepositories, IntelloService, UserAnswer,
};

// Re-export course module types for backward compatibility
