//! Order Phrase Repository Trait - Abstraction for order phrase set persistence

use crate::domain::intello::OrderPhraseSet;
use crate::error::IntelloError;
use async_trait::async_trait;

/// Repository trait for order phrase set persistence operations
#[async_trait]
pub trait OrderPhraseRepository: Send + Sync {
    async fn insert(&self, set: &OrderPhraseSet) -> Result<OrderPhraseSet, IntelloError>;
    async fn find_by_id(&self, set_id: &str, user_id: &str) -> Result<Option<OrderPhraseSet>, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OrderPhraseSet>, IntelloError>;
    async fn delete(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError>;
}
