//! Supabase integration - Authentication and database services
//!
//! This module provides:
//! - `SupabaseClient` - Authentication operations (login, register, logout)
//! - `SupabaseAppRepository` - App database operations
//! - `SupabaseUserAppRepository` - User app database operations
//! - `SupabaseCollectionRepository` - Collection management operations
//! - `SupabaseDvdRepository` - DVD database operations
//! - `SupabaseQcmRepository` - QCM set database operations
//! - `SupabaseOpenQuestionRepository` - Open question set database operations
//! - `SupabaseFlashcardRepository` - Flashcard set database operations
//! - `SupabaseTrueOrFalseRepository` - True or False set database operations
//! - `SupabaseKeywordsRepository` - Keywords set database operations
//! - `SupabaseCourseRepository` - Course, resource, and session management operations
//! - `SupabaseAiUsageRepository` - AI usage tracking for cost monitoring

mod ai_usage;
mod app;
mod client;
mod collection;
mod course;
mod dvd;
mod fill_blank;
mod flashcard;
mod keywords;
mod open_question;
mod order_phrase;
mod qcm;
mod shared;
mod study_session;
mod true_false;
mod user_app;
pub mod error;

pub use shared::SupabaseHttpClient;
pub use error::SupabaseError;

pub use ai_usage::SupabaseAiUsageRepository;
pub use app::SupabaseAppRepository;
pub use client::SupabaseClient;
pub use collection::SupabaseCollectionRepository;
pub use course::SupabaseCourseRepository;
pub use dvd::SupabaseDvdRepository;
pub use fill_blank::SupabaseFillBlankRepository;
pub use flashcard::SupabaseFlashcardRepository;
pub use keywords::SupabaseKeywordsRepository;
pub use open_question::SupabaseOpenQuestionRepository;
pub use order_phrase::SupabaseOrderPhraseRepository;
pub use qcm::SupabaseQcmRepository;
pub use study_session::SupabaseStudySessionRepository;
pub use true_false::SupabaseTrueOrFalseRepository;
pub use user_app::SupabaseUserAppRepository;
