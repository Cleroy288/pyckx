//! Flashcard Repository Trait - Abstraction for flashcard set persistence

use crate::domain::intello::FlashcardSet;
use crate::error::IntelloError;
use async_trait::async_trait;

/// Repository trait for flashcard set persistence operations
#[async_trait]
#[allow(dead_code)]
pub trait FlashcardRepository: Send + Sync {
    async fn insert(&self, set: &FlashcardSet) -> Result<FlashcardSet, IntelloError>;
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<FlashcardSet>, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<FlashcardSet>, IntelloError>;
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError>;
}
