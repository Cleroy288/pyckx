//! QCM Repository Trait - Abstraction for QCM set persistence operations

use crate::services::intello::domain::error_domain::IntelloError;
use crate::services::intello::QcmSet;
use async_trait::async_trait;

/// Repository trait for QCM set persistence operations
#[async_trait]
pub trait QcmRepository: Send + Sync {
    async fn insert(&self, qcm_set: &QcmSet) -> Result<QcmSet, IntelloError>;
    async fn find_by_id(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<QcmSet>, IntelloError>;
    async fn find_by_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<QcmSet>, IntelloError>;
    async fn update(&self, qcm_set: &QcmSet) -> Result<bool, IntelloError>;
    async fn delete(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<bool, IntelloError>;
}
