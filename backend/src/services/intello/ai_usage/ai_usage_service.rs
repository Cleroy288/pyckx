//! AI Usage Operations - Logging and cost tracking

use tracing::{info, instrument, warn};

use super::ai_usage_domain::{AiUsageLog, CreateAiUsageLog};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::IntelloService;
use crate::infra::openrouter::calculate_cost;

impl IntelloService {
    // ** log_ai_usage **
    // ==> Logs AI usage with automatic cost calculation
    //
    // @ user_id : The user who initiated the AI request
    // @ model_id : The AI model used (e.g., "gpt-4")
    // @ feature_type : Feature that used AI (e.g., "flashcard", "qcm")
    // @ input_tokens : Number of input tokens consumed
    // @ output_tokens : Number of output tokens generated
    // @ returns : The created AiUsageLog record
    // @ errors : StorageError if logging fails
    #[instrument(skip(self), fields(user_id = %user_id, model = %model_id, feature = %feature_type))]
    pub async fn log_ai_usage(
        &self,
        user_id: &str,
        model_id: &str,
        feature_type: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) -> Result<AiUsageLog, IntelloError> {
        // Step 1: Calculate cost using centralized model registry
        let cost = calculate_cost(model_id, input_tokens, output_tokens);

        // Step 2: Build usage log input
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

        // Step 3: Persist usage log to database
        let log = self
            .ai_usage_repo
            .log_usage(input)
            .await
            .map_err(|e| IntelloError::storage(e.to_string()))?;

        // Step 4: Log the operation
        info!(
            input_tokens = input_tokens,
            output_tokens = output_tokens,
            total_cost = cost.total_cost_usd,
            "AI usage logged"
        );

        // Step 5: Return the created log
        Ok(log)
    }

    // ** try_log_ai_usage **
    // ==> Fire-and-forget AI usage logging (doesn't fail on errors)
    //
    // @ user_id : The user who initiated the AI request
    // @ model_id : The AI model used
    // @ feature_type : Feature that used AI
    // @ input_tokens : Number of input tokens consumed
    // @ output_tokens : Number of output tokens generated
    pub async fn try_log_ai_usage(
        &self,
        user_id: &str,
        model_id: &str,
        feature_type: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) {
        // Step 1: Attempt to log usage, suppress errors
        if let Err(e) = self
            .log_ai_usage(user_id, model_id, feature_type, input_tokens, output_tokens)
            .await
        {
            // Step 2: Log warning if logging failed
            warn!("Failed to log AI usage: {}", e);
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
            .map_err(|e| IntelloError::storage(e.to_string()))
    }
}
