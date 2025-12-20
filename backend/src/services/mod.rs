//! Services layer - Business logic orchestration.
//!
//! Services coordinate between domain entities and infrastructure.
//! They contain the core business rules and workflows.
//!
//! # Structure
//! - `auth/` - Authentication service (login, register, logout)
//! - `apps/` - App and user app management
//! - `collection/` - Collection service (DVD management)
//! - `intello/` - Intello service (QCM, Open Questions, Flashcards)
//! - `openrouter/` - OpenRouter AI service

mod apps;
mod auth;
mod collection;
mod intello;
mod openrouter;

pub use apps::AppService;
pub use auth::AuthService;
pub use collection::CollectionService;
pub use intello::{
    AnswerGrade as IntelloAnswerGrade, CheckAnswersInput, GenerateContentInput, GradingResult,
    IntelloRepositories, IntelloService, UserAnswer,
};
pub use openrouter::{AnswerGrade, GradedAnswer, OpenRouterService, AVAILABLE_MODELS, DEFAULT_MODEL, validate_model, validate_token_count};

