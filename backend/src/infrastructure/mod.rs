//! Infrastructure layer - External service integrations.
//!
//! This layer handles all communication with external services like databases,
//! third-party APIs, message queues, etc. It translates between external
//! formats and domain types.
//!
//! # Structure
//! - `supabase/` - Supabase authentication and database client
//! - `repository/` - Repository traits (abstractions)

pub mod repository;
pub mod supabase;

// Repository traits
pub use repository::{
    AppRepository, CollectionRepository, CreateApp, CreateDvd, 
    DvdRepository, FillBlankRepository, FlashcardRepository, GameSetRepository, 
    KeywordsRepository, OpenQuestionRepository, OrderPhraseRepository, QcmRepository, 
    TrueOrFalseRepository, UpdateApp, UpdateDvd, UserAppRepository,
    CourseRepository,
};

// Supabase implementations
pub use supabase::{
    SupabaseAppRepository, SupabaseClient, SupabaseCollectionRepository, 
    SupabaseDvdRepository, SupabaseFillBlankRepository, 
    SupabaseFlashcardRepository, SupabaseKeywordsRepository, SupabaseOpenQuestionRepository, 
    SupabaseOrderPhraseRepository, SupabaseQcmRepository, SupabaseTrueOrFalseRepository, 
    SupabaseUserAppRepository, SupabaseCourseRepository,
};
