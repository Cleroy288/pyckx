//! Domain layer - Pure business entities with no external dependencies.
//!
//! This layer contains the core data structures that represent your business domain.
//! These types should be framework-agnostic and contain no HTTP, database, or external service logic.
//!
//! # Structure
//! - `apps/` - App entities (App, UserApp, AppInstance)
//! - `collection/` - Collection items (DVD, Book, etc.) with polymorphic support
//! - `intello/` - Intello quiz/learning app entities (QcmSet, QcmQuestion, etc.)
//! - `session/` - Session management
//! - `user/` - User entity

pub mod apps;
pub mod collection;
pub mod intello;
pub mod session;
pub mod user;

#[allow(unused_imports)]
pub use apps::{App as AppEntity, AppId, AppInstance, AppModule, UserApp};
// Collection entities
#[allow(unused_imports)]
pub use collection::{CollectionItem, CollectionItemType, Dvd, UserCollection};
// Intello entities - re-exported for convenience
#[allow(unused_imports)]
pub use intello::{Level, QcmQuestion, QcmSet};
pub use session::SessionStore;
#[allow(unused_imports)]
pub use user::{User, UserId};
