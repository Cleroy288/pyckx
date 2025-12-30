//! Admin Statistics DTOs

use serde::Serialize;

/// Response for admin statistics endpoint
#[derive(Debug, Serialize)]
pub struct AdminStatsResponse {
    pub total_requests: i64,
    pub total_cost_usd: f64,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub by_feature_type: Vec<FeatureStats>,
    pub by_model: Vec<ModelStats>,
}

/// Statistics for a specific feature type
#[derive(Debug, Serialize)]
pub struct FeatureStats {
    pub feature_type: String,
    pub count: i64,
    pub total_cost_usd: f64,
    pub avg_cost_usd: f64,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
}

/// Statistics for a specific AI model
#[derive(Debug, Serialize)]
pub struct ModelStats {
    pub model_id: String,
    pub count: i64,
    pub total_cost_usd: f64,
    pub avg_cost_usd: f64,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
}
