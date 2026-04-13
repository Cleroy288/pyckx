//! Open Question Repository Trait - Abstraction for open question set persistence

use crate::services::domain::error_domain::StudyError;
use crate::services::OpenQuestionSet;
use async_trait::async_trait;

/// Repository trait for open question set persistence operations
#[async_trait]
pub trait OpenQuestionRepository: Send + Sync {
    async fn insert(
        &self,
        set: &OpenQuestionSet,
    ) -> Result<OpenQuestionSet, StudyError>;
    async fn find_by_id(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<OpenQuestionSet>, StudyError>;
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<OpenQuestionSet>, StudyError>;
}
