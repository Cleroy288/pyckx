//! True or False Repository Trait - Abstraction for true/false set persistence

use crate::domain::intello::TrueOrFalseSet;
use crate::error::IntelloError;
use async_trait::async_trait;

/// Repository trait for true/false set persistence operations
#[async_trait]
pub trait TrueOrFalseRepository: Send + Sync {
    async fn insert(&self, set: &TrueOrFalseSet) -> Result<TrueOrFalseSet, IntelloError>;
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<TrueOrFalseSet>, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<TrueOrFalseSet>, IntelloError>;
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError>;
}
