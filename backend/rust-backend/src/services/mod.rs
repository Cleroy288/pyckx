//! Services module
//!
//! Business logic layer - service operations for each app

pub mod app_registry;
pub mod auth;
pub mod collection;

pub mod ai_response;
pub mod ai_usage;
pub mod app;
pub mod course;
pub mod crud;
pub mod document_extractor;
pub mod domain;
pub mod games;
pub mod learn_mode;
pub mod study_service;
pub mod study_session;
pub mod tracking;
pub mod utils;

#[cfg(test)]
pub mod test_support;

// Re-export commonly used types
pub use app_registry::AppService;
pub use auth::AuthService;
pub use collection::CollectionService;

// Backward-compatible re-exports for domain submodules
pub use domain::custom_question_domain;
pub use domain::error_domain;
pub use domain::types_domain;

// Re-export service struct and builder
pub use study_service::{
    StudyRepositories, StudyService,
    StudyServiceBuilder,
};

// Re-export domain IDs
pub use domain::ids_domain::{
    OptionId, QuestionId, SetId,
};

// Re-export app types
pub use app::app_domain::StudyApp;
pub use app::games_registry_domain::{
    GameInstance, AVAILABLE_GAMES,
};

// Re-export enums
pub use domain::enums_domain::Level;

// Re-export custom question types
pub use domain::custom_question_domain::{
    CustomQuestion, DocumentType,
};

// Re-export domain input/output types
pub use domain::types_domain::{
    CheckAnswersInput, DocumentList,
    GenerateContentInput, GradingResult, UserAnswer,
};

// Re-export game set types
pub use games::fill_blank::FillBlankSet;
pub use games::flashcard::FlashcardSet;
pub use games::keywords::KeywordSet;
pub use games::open_question::OpenQuestionSet;
pub use games::order_phrase::OrderPhraseSet;
pub use games::qcm::QcmSet;
pub use games::true_false::TrueOrFalseSet;

// Re-export QCM types
pub use games::qcm::CreateQcmInput;
pub use games::qcm::QcmQuestion;
pub use games::qcm::QuestionInput;
pub use games::qcm::QuickQcmInput;

// Re-export question/option types
pub use games::fill_blank::{
    FillBlankOption, FillBlankQuestion,
};
pub use games::flashcard::Flashcard;
pub use games::keywords::{Keyword, KeywordQuestion};
pub use games::open_question::OpenQuestion;
pub use games::order_phrase::{
    OrderPhraseQuestion, OrderPhraseWord,
};
pub use games::true_false::TrueOrFalseStatement;

// Re-export shared game types
pub use games::shared::types::{
    AnswerGrade, GradedAnswer,
};
