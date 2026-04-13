/*
 * Supabase AI Usage Repository
 *
 * Tracks AI model usage and costs for Study features.
 * Uses shared SupabaseHttpClient for connection pooling.
 *
 * Tables: study_ai_usage_log
 */

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

use crate::infra::database::AiUsageRepository;
use crate::infra::supabase::shared::{SupabaseError, SupabaseHttpClient};
use crate::services::ai_usage::ai_usage_domain::{
    AiUsageLog, CreateAiUsageLog,
};
use crate::shared::AppError;

/* ============================================================================
 * CONSTANTS
 * ============================================================================ */

const TABLE_AI_USAGE: &str = "intello_ai_usage_log";

/* ============================================================================
 * ROW TYPES
 * ============================================================================ */

/*
 * Row type for inserting AI usage records.
 */
#[derive(Debug, Serialize)]
struct InsertAiUsageRow {
    id: String,
    user_id: String,
    model_id: String,
    feature_type: String,
    input_tokens: i32,
    output_tokens: i32,
    input_cost_usd: f64,
    output_cost_usd: f64,
    total_cost_usd: f64,
}

/*
 * Row type for reading AI usage records.
 */
#[derive(Debug, Deserialize)]
struct AiUsageRow {
    id: String,
    user_id: String,
    model_id: String,
    feature_type: String,
    input_tokens: i32,
    output_tokens: i32,
    input_cost_usd: f64,
    output_cost_usd: f64,
    total_cost_usd: f64,
    created_at: String,
}

/* ============================================================================
 * SUPABASE AI USAGE REPOSITORY
 * ============================================================================ */

/*
 * Repository for AI usage tracking operations.
 *
 * Logs AI model usage, retrieves usage history, and calculates costs.
 */
#[derive(Clone, Debug)]
pub struct SupabaseAiUsageRepository {
    client: Arc<SupabaseHttpClient>,
}

/* ============================================================================
 * CONSTRUCTOR
 * ============================================================================ */

impl SupabaseAiUsageRepository {
    /*
     * Create repository with shared HTTP client.
     */
    pub fn new(client: Arc<SupabaseHttpClient>) -> Self {
        info!("SupabaseAiUsageRepository initialized with shared client");
        Self { client }
    }

    /* ========================================================================
     * HELPER: Error Mapping
     * ======================================================================== */

    /*
     * Convert SupabaseError to AppError.
     */
    fn map_error(err: SupabaseError) -> AppError {
        AppError::Internal(crate::http_api::utils::InternalError::new(
            err.to_string(),
        ))
    }

    /* ========================================================================
     * HELPER: Row Conversion
     * ======================================================================== */

    /*
     * Convert database row to domain entity.
     */
    fn row_to_domain(row: AiUsageRow) -> AiUsageLog {
        AiUsageLog {
            id: row.id,
            user_id: row.user_id,
            model_id: row.model_id,
            feature_type: row.feature_type,
            input_tokens: row.input_tokens,
            output_tokens: row.output_tokens,
            input_cost_usd: row.input_cost_usd,
            output_cost_usd: row.output_cost_usd,
            total_cost_usd: row.total_cost_usd,
            created_at: row.created_at,
        }
    }
}

/* ============================================================================
 * REPOSITORY IMPLEMENTATION
 * ============================================================================ */

#[async_trait]
impl AiUsageRepository for SupabaseAiUsageRepository {
    /* ========================================================================
     * CREATE: Log AI usage
     * ======================================================================== */

    /*
     * Log a new AI usage record.
     *
     * Creates a usage log entry with token counts and calculated costs.
     */
    async fn log_usage(
        &self,
        input: CreateAiUsageLog,
    ) -> Result<AiUsageLog, AppError> {
        let id = Uuid::new_v4().to_string();
        let row = InsertAiUsageRow {
            id: id.clone(),
            user_id: input.user_id.to_string(),
            model_id: input.model_id.clone(),
            feature_type: input.feature_type.clone(),
            input_tokens: input.input_tokens,
            output_tokens: input.output_tokens,
            input_cost_usd: input.input_cost_usd,
            output_cost_usd: input.output_cost_usd,
            total_cost_usd: input.total_cost_usd,
        };

        let url = self.client.rest_url(TABLE_AI_USAGE);
        let created: Vec<AiUsageRow> = self
            .client
            .post(&url, &row)
            .await
            .map_err(Self::map_error)?;

        let row = created.into_iter().next().ok_or_else(|| {
            AppError::Internal(crate::http_api::utils::InternalError::new(
                "No usage returned".to_string(),
            ))
        })?;

        info!(
            model = %row.model_id,
            feature = %row.feature_type,
            input_tokens = row.input_tokens,
            output_tokens = row.output_tokens,
            cost = row.total_cost_usd,
            "AI usage logged"
        );

        Ok(Self::row_to_domain(row))
    }

    /* ========================================================================
     * READ: Get user usage history
     * ======================================================================== */

    /*
     * Get all AI usage records for a user.
     *
     * Returns usage history ordered by most recent first.
     */
    async fn get_user_usage(
        &self,
        user_id: &str,
    ) -> Result<Vec<AiUsageLog>, AppError> {
        let query = format!("user_id=eq.{}&order=created_at.desc", user_id);
        let url = self.client.rest_url_with_query(TABLE_AI_USAGE, &query);

        let rows: Vec<AiUsageRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        Ok(rows.into_iter().map(Self::row_to_domain).collect())
    }

    /* ========================================================================
     * READ: Get all usage logs (admin)
     * ======================================================================== */

    /*
     * Get all AI usage records for admin statistics.
     *
     * Returns all usage records ordered by most recent first.
     */
    async fn get_all_usage(&self) -> Result<Vec<AiUsageLog>, AppError> {
        let query = "order=created_at.desc";
        let url = self.client.rest_url_with_query(TABLE_AI_USAGE, query);

        let rows: Vec<AiUsageRow> =
            self.client.get(&url).await.map_err(Self::map_error)?;

        Ok(rows.into_iter().map(Self::row_to_domain).collect())
    }
}
