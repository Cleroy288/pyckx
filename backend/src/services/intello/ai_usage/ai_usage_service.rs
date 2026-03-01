//! AI Usage Operations - Logging and cost tracking

use tracing::{info, instrument, warn};

use super::ai_usage_domain::{AiUsageInput, AiUsageLog, CreateAiUsageLog};
use crate::infra::openrouter::calculate_cost;
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;

impl IntelloService {
    // ** log_ai_usage **
    // ==> Logs AI usage with automatic cost calculation
    //
    // @ input : AiUsageInput with user, model, feature, and token counts
    // @ returns : The created AiUsageLog record
    // @ errors : StorageError if logging fails
    #[instrument(skip(self, input), fields(user_id = %input.user_id, model = %input.model_id, feature = %input.feature_type))]
    pub async fn log_ai_usage(
        &self,
        input: AiUsageInput<'_>,
    ) -> Result<AiUsageLog, IntelloError> {
        // Step 1: Calculate cost using centralized model registry
        let cost = calculate_cost(
            input.model_id,
            input.input_tokens,
            input.output_tokens,
        );

        // Step 2: Build usage log entry
        let create = CreateAiUsageLog {
            user_id: input.user_id.to_string(),
            model_id: input.model_id.to_string(),
            feature_type: input.feature_type.to_string(),
            input_tokens: input.input_tokens as i32,
            output_tokens: input.output_tokens as i32,
            input_cost_usd: cost.input_cost_usd,
            output_cost_usd: cost.output_cost_usd,
            total_cost_usd: cost.total_cost_usd,
        };

        // Step 3: Persist usage log to database
        let log = self
            .ai_usage_repo
            .log_usage(create)
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))?;

        // Step 4: Log the operation
        info!(
            input_tokens = input.input_tokens,
            output_tokens = input.output_tokens,
            total_cost = cost.total_cost_usd,
            "AI usage logged"
        );

        // Step 5: Return the created log
        Ok(log)
    }

    // ** try_log_ai_usage **
    // ==> Fire-and-forget AI usage logging (doesn't fail)
    //
    // @ input : AiUsageInput with user, model, feature, and token counts
    pub async fn try_log_ai_usage(&self, input: AiUsageInput<'_>) {
        // Step 1: Attempt to log usage, suppress errors
        if let Err(err) = self.log_ai_usage(input).await {
            // Step 2: Log warning if logging failed
            warn!("Failed to log AI usage: {}", err);
        }
    }

    // ** get_all_usage **
    // ==> Retrieves all AI usage logs for admin statistics
    //
    // @ returns : Vector of all AiUsageLog records
    // @ errors : StorageError if query fails
    pub async fn get_all_usage(&self) -> Result<Vec<AiUsageLog>, IntelloError> {
        // Step 1: Query repository for all usage logs
        self.ai_usage_repo
            .get_all_usage()
            .await
            .map_err(|err| IntelloError::storage(err.to_string()))
    }
}
