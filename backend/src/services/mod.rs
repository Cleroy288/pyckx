//! Services module
//!
//! Business logic layer - service operations for each app

pub mod app_registry;
pub mod auth;
pub mod collection;
pub mod intello;

// Re-export commonly used types
pub use app_registry::AppService;
pub use auth::AuthService;
pub use collection::CollectionService;
pub use intello::{
    CheckAnswersInput, GenerateContentInput, IntelloRepositories, IntelloService, UserAnswer,
};
