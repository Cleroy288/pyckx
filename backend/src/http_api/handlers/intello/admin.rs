//! Admin Statistics Handler
//!
//! Provides aggregate statistics for AI usage.
//! SECURITY: Only accessible by ADMIN_EMAIL from environment variable

use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;
use std::env;
use tracing::info;

use crate::app::App;
use crate::http_api::data_transfer_object::intello::admin::{AdminStatsResponse, FeatureStats};

/// Get admin email from environment variable
fn get_admin_email() -> String {
    env::var("ADMIN_EMAIL").unwrap_or_else(|_| String::new())
}

/// Helper to extract session_id from cookies
fn extract_session_id(req: &HttpRequest) -> Option<String> {
    let cookies = req.cookies().ok()?;
    cookies
        .iter()
        .find(|c| c.name() == "session_id")
        .map(|c| c.value().to_string())
}

/// GET /api/intello/admin/stats
///
/// Returns aggregate AI usage statistics.
/// Only accessible by admin user.
#[get("/admin/stats")]
pub async fn get_admin_stats(app: Data<App>, req: HttpRequest) -> impl Responder {
    // Extract session_id from cookie
    let session_id = match extract_session_id(&req) {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Not authenticated. Please log in."
            }));
        }
    };

    // Get user from session
    let user = match app.auth.sessions().get_user(&session_id) {
        Some(u) => u,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Session expired. Please log in again."
            }));
        }
    };

    // Check admin access
    let admin_email = get_admin_email();
    if admin_email.is_empty() || user.email != admin_email {
        return HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied. Admin only."
        }));
    }

    info!(email = %user.email, "Admin stats requested");

    // Fetch all usage logs
    let usage_logs = match app.intello_service.get_all_usage().await {
        Ok(logs) => logs,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch usage: {}", e)
            }));
        }
    };

    // Calculate aggregates
    let total_requests = usage_logs.len() as i64;
    let total_cost_usd: f64 = usage_logs.iter().map(|l| l.total_cost_usd).sum();
    let total_input_tokens: i64 = usage_logs.iter().map(|l| l.input_tokens as i64).sum();
    let total_output_tokens: i64 = usage_logs.iter().map(|l| l.output_tokens as i64).sum();

    // Group by feature type
    let mut by_feature: HashMap<String, (i64, f64, i64, i64)> = HashMap::new();
    for log in &usage_logs {
        let entry = by_feature
            .entry(log.feature_type.clone())
            .or_insert((0, 0.0, 0, 0));
        entry.0 += 1;
        entry.1 += log.total_cost_usd;
        entry.2 += log.input_tokens as i64;
        entry.3 += log.output_tokens as i64;
    }

    let mut by_feature_type: Vec<FeatureStats> = by_feature
        .into_iter()
        .map(
            |(feature_type, (count, total_cost, input_tokens, output_tokens))| FeatureStats {
                feature_type,
                count,
                total_cost_usd: total_cost,
                avg_cost_usd: if count > 0 {
                    total_cost / count as f64
                } else {
                    0.0
                },
                total_input_tokens: input_tokens,
                total_output_tokens: output_tokens,
            },
        )
        .collect();

    // Sort by total cost descending
    by_feature_type.sort_by(|a, b| {
        b.total_cost_usd
            .partial_cmp(&a.total_cost_usd)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Group by model
    let mut by_model_map: HashMap<String, (i64, f64, i64, i64)> = HashMap::new();
    for log in &usage_logs {
        let entry = by_model_map
            .entry(log.model_id.clone())
            .or_insert((0, 0.0, 0, 0));
        entry.0 += 1;
        entry.1 += log.total_cost_usd;
        entry.2 += log.input_tokens as i64;
        entry.3 += log.output_tokens as i64;
    }

    let mut by_model: Vec<crate::http_api::data_transfer_object::intello::admin::ModelStats> =
        by_model_map
            .into_iter()
            .map(
                |(model_id, (count, total_cost, input_tokens, output_tokens))| {
                    crate::http_api::data_transfer_object::intello::admin::ModelStats {
                        model_id,
                        count,
                        total_cost_usd: total_cost,
                        avg_cost_usd: if count > 0 {
                            total_cost / count as f64
                        } else {
                            0.0
                        },
                        total_input_tokens: input_tokens,
                        total_output_tokens: output_tokens,
                    }
                },
            )
            .collect();

    // Sort by count descending (most used first)
    by_model.sort_by(|a, b| b.count.cmp(&a.count));

    let response = AdminStatsResponse {
        total_requests,
        total_cost_usd,
        total_input_tokens,
        total_output_tokens,
        by_feature_type,
        by_model,
    };

    HttpResponse::Ok().json(response)
}
