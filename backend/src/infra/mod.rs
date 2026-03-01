//! Infrastructure layer - External service integrations and shared types.
//!
//! This layer handles all communication with external services like databases,
//! third-party APIs, message queues, etc. It also contains shared infrastructure
//! types used across the application.
//!
//! # Structure
//! - `supabase/` - Supabase authentication and database client (includes UserId)
//! - `database/` - Database repository traits (abstractions)
//! - `user` - User entity with auth tokens
//! - `session` - Session management with CSV persistence

pub mod database;
pub mod openrouter; // OpenRouter HTTP client
pub mod session;
pub mod supabase;
pub mod user;

// Session and User
pub use session::SessionStore;
pub use user::User;

// Repository traits
pub use database::{
    AiUsageRepository, AppRepository, CollectionRepository, CourseRepository,
    CreateApp, CreateDvd, DvdRepository, FillBlankRepository,
    FlashcardRepository, GameSetRepository, KeywordsRepository,
    OpenQuestionRepository, OrderPhraseRepository, QcmRepository,
    StudySessionRepository, TrueOrFalseRepository, UpdateApp, UpdateDvd,
    UserAppRepository,
};

// Supabase implementations
pub use supabase::{
    SupabaseAiUsageRepository, SupabaseAppRepository, SupabaseClient,
    SupabaseCollectionRepository, SupabaseCourseRepository,
    SupabaseDvdRepository, SupabaseFillBlankRepository,
    SupabaseFlashcardRepository, SupabaseHttpClient,
    SupabaseKeywordsRepository, SupabaseOpenQuestionRepository,
    SupabaseOrderPhraseRepository, SupabaseQcmRepository,
    SupabaseSessionRepository, SupabaseStudySessionRepository,
    SupabaseTrueOrFalseRepository, SupabaseUserAppRepository,
};
