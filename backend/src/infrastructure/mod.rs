//! Infrastructure layer - External service integrations.
//!
//! This layer handles all communication with external services like databases,
//! third-party APIs, message queues, etc. It translates between external
//! formats and domain types.
//!
//! # Structure
//! - `supabase/` - Supabase authentication and database client
//! - `repository/` - Repository traits (abstractions)
//! - `json_storage/` - JSON file-based storage implementations

pub mod json_storage;
pub mod repository;
pub mod supabase;

// Repository traits
pub use repository::{
    AppRepository, CollectionRepository, CreateApp, CreateDvd, DvdRepository, FlashcardRepository,
    KeywordsRepository, OpenQuestionRepository, OrderPhraseRepository, QcmRepository, TrueOrFalseRepository, UpdateApp, UpdateDvd, UserAppRepository,
};

// JSON storage implementations (legacy, used in tests only)
#[allow(unused_imports)]
pub use json_storage::{JsonFlashcardRepository, JsonOpenQuestionRepository, JsonQcmRepository};

// Supabase implementations
pub use supabase::{
    SupabaseAppRepository, SupabaseClient, SupabaseCollectionRepository, SupabaseDvdRepository,
    SupabaseFlashcardRepository, SupabaseKeywordsRepository, SupabaseOpenQuestionRepository, SupabaseOrderPhraseRepository, SupabaseQcmRepository,
    SupabaseTrueOrFalseRepository, SupabaseUserAppRepository,
};

