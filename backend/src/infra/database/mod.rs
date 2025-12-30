//! Repository layer - Database abstraction traits
//!
//! This module defines repository traits that abstract database operations.
//! Following the Dependency Inversion Principle, services depend on these
//! traits rather than concrete implementations.
//!
//! # Structure
//! - `game_set/` - Generic game set repository trait (shared by all Intello games)
//! - `app/` - App repository trait and DTOs
//! - `user_app/` - User app repository trait
//! - `collection/` - Collection repository trait
//! - `dvd/` - DVD repository trait and DTOs
//! - `qcm/` - QCM repository trait (Intello app)
//! - `open_question/` - Open question repository trait (Intello app)
//! - `flashcard/` - Flashcard repository trait (Intello app)
//! - `true_false/` - True or False repository trait (Intello app)
//! - `keywords/` - Keywords repository trait (Intello app)
//! - `ai_usage/` - AI usage tracking repository (Intello app)

mod ai_usage;
mod app;
mod collection;
pub mod course;
mod dvd;
mod fill_blank;
mod flashcard;
mod game_set;
mod keywords;
mod open_question;
mod order_phrase;
mod qcm;
mod study_session;
mod true_false;
mod user_app;

// Generic game set repository trait
pub use game_set::GameSetRepository;

// App repositories
pub use app::{AppRepository, CreateApp, UpdateApp};
pub use collection::CollectionRepository;
pub use dvd::{CreateDvd, DvdRepository, UpdateDvd};
pub use user_app::UserAppRepository;

// Intello game repositories
pub use ai_usage::AiUsageRepository;
pub use course::CourseRepository;
pub use fill_blank::FillBlankRepository;
pub use flashcard::FlashcardRepository;
pub use keywords::KeywordsRepository;
pub use open_question::OpenQuestionRepository;
pub use order_phrase::OrderPhraseRepository;
pub use qcm::QcmRepository;
pub use study_session::StudySessionRepository;
pub use true_false::TrueOrFalseRepository;
