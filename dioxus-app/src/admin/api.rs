//! Admin API — types and stats fetcher.

use crate::api::{endpoints, helpers};
use serde::Deserialize;

/// Admin dashboard statistics payload.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AdminStats {
    pub total_users: usize,
    pub total_generations: usize,
    pub features: Vec<FeatureStats>,
    pub models: Vec<ModelStats>,
}

/// Per-feature usage stats.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FeatureStats {
    pub feature: String,
    pub count: usize,
}

/// Per-model usage stats.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ModelStats {
    pub model: String,
    pub count: usize,
    pub total_tokens: usize,
}

/// Fetch the admin dashboard statistics.
pub async fn fetch_stats() -> Result<AdminStats, String> {
    helpers::get_json(endpoints::ADMIN_STATS).await
}

/// Format a model row as `"<N> calls, <M> tokens"`.
pub fn format_model_line(stats: &ModelStats) -> String {
    format!("{} calls, {} tokens", stats.count, stats.total_tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_admin_stats_full_payload() {
        let json = r#"{
            "total_users": 42,
            "total_generations": 100,
            "features": [
                {"feature": "qcm", "count": 50}
            ],
            "models": [
                {"model": "gpt-4", "count": 30,
                 "total_tokens": 9000}
            ]
        }"#;
        let stats: AdminStats =
            serde_json::from_str(json).unwrap();
        assert_eq!(stats.total_users, 42);
    }

    #[test]
    fn test_deserialize_admin_stats_empty_lists() {
        let json = r#"{
            "total_users": 0,
            "total_generations": 0,
            "features": [],
            "models": []
        }"#;
        let stats: AdminStats =
            serde_json::from_str(json).unwrap();
        assert!(stats.features.is_empty());
    }

    #[test]
    fn test_format_model_line_builds_readable_string() {
        let stats = ModelStats {
            model: "gpt-4".to_string(),
            count: 12,
            total_tokens: 3_400,
        };
        assert_eq!(
            format_model_line(&stats),
            "12 calls, 3400 tokens",
        );
    }

    #[test]
    fn test_format_model_line_with_zero_values() {
        let stats = ModelStats {
            model: "none".to_string(),
            count: 0,
            total_tokens: 0,
        };
        assert_eq!(
            format_model_line(&stats),
            "0 calls, 0 tokens",
        );
    }
}
