//! AI Usage Operations - Logging and cost tracking

use tracing::{info, instrument, warn};

use crate::services::intello::ai_usage_domain::{AiUsageLog, CreateAiUsageLog};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;
use crate::services::openrouter::models_domain::calculate_cost;

impl IntelloService {
    /// Log AI usage with automatic cost calculation
    #[instrument(skip(self), fields(user_id = %user_id, model = %model_id, feature = %feature_type))]
    pub async fn log_ai_usage(
        &self,
        user_id: &str,
        model_id: &str,
        feature_type: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) -> Result<AiUsageLog, IntelloError> {
        // Calculate cost using centralized model registry
        let cost = calculate_cost(model_id, input_tokens, output_tokens);

        let input = CreateAiUsageLog {
            user_id: user_id.to_string(),
            model_id: model_id.to_string(),
            feature_type: feature_type.to_string(),
            input_tokens: input_tokens as i32,
            output_tokens: output_tokens as i32,
            input_cost_usd: cost.input_cost_usd,
            output_cost_usd: cost.output_cost_usd,
            total_cost_usd: cost.total_cost_usd,
        };

        let log = self
            .ai_usage_repo
            .log_usage(input)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

        info!(
            input_tokens = input_tokens,
            output_tokens = output_tokens,
            total_cost = cost.total_cost_usd,
            "AI usage logged"
        );

        Ok(log)
    }

    /// Log AI usage, but don't fail if logging fails (fire-and-forget)
    pub async fn try_log_ai_usage(
        &self,
        user_id: &str,
        model_id: &str,
        feature_type: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) {
        if let Err(e) = self
            .log_ai_usage(user_id, model_id, feature_type, input_tokens, output_tokens)
            .await
        {
            warn!("Failed to log AI usage: {}", e);
        }
    }

    /// Get all AI usage logs for admin statistics
    pub async fn get_all_usage(&self) -> Result<Vec<AiUsageLog>, IntelloError> {
        self.ai_usage_repo
            .get_all_usage()
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))
    }
}
