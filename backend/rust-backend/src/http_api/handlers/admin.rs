//! Admin Statistics Handler
//!
//! Provides aggregate statistics for AI usage.
//! SECURITY: Only accessible by ADMIN_EMAIL env var.

use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use std::env;
use tracing::info;

use crate::app::App;
use crate::http_api::data_transfer_object::admin::{
    AdminStatsResponse, FeatureStats, ModelStats,
};
use crate::services::ai_usage::ai_usage_domain::{
    AggregatedUsageStats, GroupedStats,
};

/// Get admin email from environment variable
fn get_admin_email() -> String {
    env::var("ADMIN_EMAIL")
        .unwrap_or_else(|_| String::new())
}

/// Extract session_id from cookies
fn extract_session_id(
    req: &HttpRequest,
) -> Option<String> {
    let cookies = req.cookies().ok()?;
    cookies
        .iter()
        .find(|c| c.name() == "session_id")
        .map(|c| c.value().to_string())
}

/// GET /api/study/admin/stats
///
/// Returns aggregate AI usage statistics.
/// Only accessible by admin user.
#[get("/admin/stats")]
pub async fn get_admin_stats(
    app: Data<App>,
    req: HttpRequest,
) -> impl Responder {
    // Authenticate and authorize
    let session_id = match extract_session_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": "Not authenticated."
                }));
        }
    };

    let user = match app
        .auth
        .sessions()
        .get_user(&session_id)
        .await
    {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": "Session expired."
                }));
        }
    };

    let admin_email = get_admin_email();
    if admin_email.is_empty()
        || user.email != admin_email
    {
        return HttpResponse::Forbidden()
            .json(serde_json::json!({
                "error": "Access denied. Admin only."
            }));
    }

    info!(email = %user.email, "Admin stats requested");

    // Delegate aggregation to service
    let stats = match app
        .study_service
        .get_aggregated_usage()
        .await
    {
        Ok(s) => s,
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({
                    "error":
                        format!("Failed: {}", err)
                }));
        }
    };

    HttpResponse::Ok().json(build_response(stats))
}

/// Convert domain stats to admin response DTO
fn build_response(
    stats: AggregatedUsageStats,
) -> AdminStatsResponse {
    AdminStatsResponse {
        total_requests: stats.total_requests,
        total_cost_usd: stats.total_cost_usd,
        total_input_tokens: stats.total_input_tokens,
        total_output_tokens: stats.total_output_tokens,
        by_feature_type: stats
            .by_feature
            .into_iter()
            .map(to_feature_stats)
            .collect(),
        by_model: stats
            .by_model
            .into_iter()
            .map(to_model_stats)
            .collect(),
    }
}

/// Map GroupedStats to FeatureStats DTO
fn to_feature_stats(g: GroupedStats) -> FeatureStats {
    FeatureStats {
        feature_type: g.key,
        count: g.count,
        total_cost_usd: g.total_cost_usd,
        avg_cost_usd: g.avg_cost_usd,
        total_input_tokens: g.total_input_tokens,
        total_output_tokens: g.total_output_tokens,
    }
}

/// Map GroupedStats to ModelStats DTO
fn to_model_stats(g: GroupedStats) -> ModelStats {
    ModelStats {
        model_id: g.key,
        count: g.count,
        total_cost_usd: g.total_cost_usd,
        avg_cost_usd: g.avg_cost_usd,
        total_input_tokens: g.total_input_tokens,
        total_output_tokens: g.total_output_tokens,
    }
}
