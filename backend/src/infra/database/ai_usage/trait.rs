//! AI Usage Repository - Trait definition

use crate::services::intello::ai_usage_domain::{AiUsageLog, CreateAiUsageLog};
use crate::shared::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait AiUsageRepository: Send + Sync {
    /// Log an AI usage entry
    async fn log_usage(&self, input: CreateAiUsageLog) -> Result<AiUsageLog, AppError>;

    /// Get total usage for a user
    #[allow(dead_code)] // Will be used in billing/quota dashboard
    async fn get_user_usage(&self, user_id: &str) -> Result<Vec<AiUsageLog>, AppError>;

    /// Get all usage logs (for admin stats)
    async fn get_all_usage(&self) -> Result<Vec<AiUsageLog>, AppError>;
}
