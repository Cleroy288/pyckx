//! Collection domain - Entities and traits for user collections
//!
//! This module contains the core domain entities for the collection system:
//! - `CollectionItem` trait - Common interface for all collection items
//! - `CollectionItemType` enum - Types of items (DVD, etc.)
//! - `UserCollection` - User's collection of a specific type
//! - `Dvd` - DVD entity
//!
//! The design follows SOLID principles:
//! - Single Responsibility: Each entity has one clear purpose
//! - Open/Closed: New item types can be added by implementing CollectionItem
//! - Liskov Substitution: All CollectionItem implementations are interchangeable
//! - Interface Segregation: CollectionItem only defines essential methods
//! - Dependency Inversion: Higher layers depend on CollectionItem trait

mod dvd;
mod item;
mod user_collection;

// Re-export all public types
pub use dvd::Dvd;
pub use item::{CollectionItem, CollectionItemType};
pub use user_collection::UserCollection;
