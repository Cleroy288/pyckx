//! Repository layer - Database abstraction traits
//!
//! This module defines repository traits that abstract database operations.
//! Following the Dependency Inversion Principle, services depend on these
//! traits rather than concrete implementations.
//!
//! # Structure
//! - `app/` - App repository trait and DTOs
//! - `user_app/` - User app repository trait
//! - `collection/` - Collection repository trait
//! - `dvd/` - DVD repository trait and DTOs
//! - `qcm/` - QCM repository trait (Intello app)
//! - `open_question/` - Open question repository trait (Intello app)
//! - `flashcard/` - Flashcard repository trait (Intello app)
//! - `true_false/` - True or False repository trait (Intello app)
//! - `keywords/` - Keywords repository trait (Intello app)

mod app;
mod collection;
mod dvd;
mod flashcard;
mod keywords;
mod open_question;
mod order_phrase;
mod qcm;
mod true_false;
mod user_app;

pub use app::{AppRepository, CreateApp, UpdateApp};
pub use collection::CollectionRepository;
pub use dvd::{CreateDvd, DvdRepository, UpdateDvd};
pub use flashcard::FlashcardRepository;
pub use keywords::KeywordsRepository;
pub use open_question::OpenQuestionRepository;
pub use order_phrase::OrderPhraseRepository;
pub use qcm::QcmRepository;
pub use true_false::TrueOrFalseRepository;
pub use user_app::UserAppRepository;


