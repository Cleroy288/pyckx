//! Centralized error handling with automatic tracing integration.
//!
//! Error flow: SupabaseError → AuthError → AppError → HTTP Response
//!             CollectionError → AppError → HTTP Response
//!             IntelloError → AppError → HTTP Response
//!
//! Each layer adds context and all errors are automatically logged.
//!
//! # Module Structure
//! - `shared/` - ErrorCode enum and ErrorResponse struct
//! - `supabase/` - Infrastructure layer errors
//! - `auth/` - Authentication errors
//! - `collection/` - Collection service errors
//! - `intello/` - Intello app errors
//! - `app/` - Application layer errors + ResponseError

pub mod app;
pub mod auth;
pub mod collection;
pub mod intello;
pub mod shared;
pub mod supabase;

// Re-export all public types
pub use app::{AppError, AppResult};
pub use auth::AuthError;
pub use collection::CollectionError;
pub use intello::IntelloError;
pub use supabase::SupabaseError;
