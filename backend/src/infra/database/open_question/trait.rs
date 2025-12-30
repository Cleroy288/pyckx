//! Open Question Repository Trait - Abstraction for open question set persistence

use crate::services::intello::open_question_domain::OpenQuestionSet;
use crate::services::intello::error_domain::IntelloError;
use async_trait::async_trait;

/// Repository trait for open question set persistence operations
#[async_trait]
pub trait OpenQuestionRepository: Send + Sync {
    async fn insert(&self, set: &OpenQuestionSet) -> Result<OpenQuestionSet, IntelloError>;
    async fn find_by_id(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<OpenQuestionSet>, IntelloError>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<OpenQuestionSet>, IntelloError>;
}
