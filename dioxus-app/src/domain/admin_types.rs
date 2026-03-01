//! Admin domain types — statistics

use serde::Deserialize;

/// Admin dashboard statistics
#[derive(Debug, Clone, Deserialize)]
pub struct AdminStatsResponse {
    pub total_users: usize,
    pub total_generations: usize,
    pub features: Vec<FeatureStats>,
    pub models: Vec<ModelStats>,
}

/// Per-feature usage stats
#[derive(Debug, Clone, Deserialize)]
pub struct FeatureStats {
    pub feature: String,
    pub count: usize,
}

/// Per-model usage stats
#[derive(Debug, Clone, Deserialize)]
pub struct ModelStats {
    pub model: String,
    pub count: usize,
    pub total_tokens: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_admin_stats() {
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
        let s: AdminStatsResponse =
            serde_json::from_str(json).unwrap();
        assert_eq!(s.total_users, 42);
        assert_eq!(s.features.len(), 1);
        assert_eq!(s.models[0].total_tokens, 9000);
    }

    #[test]
    fn test_deserialize_empty_stats() {
        let json = r#"{
            "total_users": 0,
            "total_generations": 0,
            "features": [],
            "models": []
        }"#;
        let s: AdminStatsResponse =
            serde_json::from_str(json).unwrap();
        assert!(s.features.is_empty());
    }
}
