//! Keywords Repository Trait - Abstraction for keyword set persistence

use crate::domain::intello::KeywordSet;
use crate::error::IntelloError;
use async_trait::async_trait;

/// Repository trait for keyword set persistence operations
#[async_trait]
pub trait KeywordsRepository: Send + Sync {
    async fn insert(&self, set: &KeywordSet) -> Result<KeywordSet, IntelloError>;
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<KeywordSet>, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<KeywordSet>, IntelloError>;
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError>;
}
