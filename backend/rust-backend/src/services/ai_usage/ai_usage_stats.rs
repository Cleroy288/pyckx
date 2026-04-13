//! AI usage statistics aggregation
//!
//! Pure functions to aggregate usage logs into stats.

use std::collections::HashMap;

use super::ai_usage_domain::{
    AggregatedUsageStats, AiUsageLog, GroupedStats,
};

/// Accumulator for usage stats per grouping key
struct Accumulator {
    count: i64,
    cost: f64,
    input_tokens: i64,
    output_tokens: i64,
}

impl Accumulator {
    fn new() -> Self {
        Self {
            count: 0,
            cost: 0.0,
            input_tokens: 0,
            output_tokens: 0,
        }
    }

    fn add(&mut self, log: &AiUsageLog) {
        self.count += 1;
        self.cost += log.total_cost_usd;
        self.input_tokens += log.input_tokens as i64;
        self.output_tokens += log.output_tokens as i64;
    }

    fn into_grouped_stats(self, key: String) -> GroupedStats {
        let avg = if self.count > 0 {
            self.cost / self.count as f64
        } else {
            0.0
        };
        GroupedStats {
            key,
            count: self.count,
            total_cost_usd: self.cost,
            avg_cost_usd: avg,
            total_input_tokens: self.input_tokens,
            total_output_tokens: self.output_tokens,
        }
    }
}

/// Aggregate usage logs into grouped stats
pub fn aggregate_usage_stats(
    logs: &[AiUsageLog],
) -> AggregatedUsageStats {
    let total_requests = logs.len() as i64;
    let total_cost_usd: f64 =
        logs.iter().map(|l| l.total_cost_usd).sum();
    let total_input_tokens: i64 =
        logs.iter().map(|l| l.input_tokens as i64).sum();
    let total_output_tokens: i64 =
        logs.iter().map(|l| l.output_tokens as i64).sum();

    let by_feature =
        group_by(logs, |l| l.feature_type.clone(), true);
    let by_model =
        group_by(logs, |l| l.model_id.clone(), false);

    AggregatedUsageStats {
        total_requests,
        total_cost_usd,
        total_input_tokens,
        total_output_tokens,
        by_feature,
        by_model,
    }
}

/// Group logs by a key, sort by cost (desc) or count (desc)
fn group_by<F>(
    logs: &[AiUsageLog],
    key_fn: F,
    sort_by_cost: bool,
) -> Vec<GroupedStats>
where
    F: Fn(&AiUsageLog) -> String,
{
    let mut map: HashMap<String, Accumulator> = HashMap::new();

    for log in logs {
        let key = key_fn(log);
        map.entry(key)
            .or_insert_with(Accumulator::new)
            .add(log);
    }

    let mut result: Vec<GroupedStats> = map
        .into_iter()
        .map(|(k, acc)| acc.into_grouped_stats(k))
        .collect();

    if sort_by_cost {
        result.sort_by(|a, b| {
            b.total_cost_usd
                .partial_cmp(&a.total_cost_usd)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        result.sort_by(|a, b| b.count.cmp(&a.count));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a test usage log
    fn test_log(
        feature: &str,
        model: &str,
        cost: f64,
    ) -> AiUsageLog {
        AiUsageLog {
            id: "id".to_string(),
            user_id: "u1".to_string(),
            model_id: model.to_string(),
            feature_type: feature.to_string(),
            input_tokens: 100,
            output_tokens: 50,
            input_cost_usd: cost * 0.6,
            output_cost_usd: cost * 0.4,
            total_cost_usd: cost,
            created_at: "2024-01-01".to_string(),
        }
    }

    #[test]
    fn test_aggregate_empty_logs_returns_zeros() {
        let stats = aggregate_usage_stats(&[]);
        assert_eq!(stats.total_requests, 0);
    }

    #[test]
    fn test_aggregate_counts_total_requests() {
        let logs = vec![
            test_log("qcm", "gpt-4", 0.01),
            test_log("flashcard", "gpt-4", 0.02),
        ];
        let stats = aggregate_usage_stats(&logs);
        assert_eq!(stats.total_requests, 2);
    }

    #[test]
    fn test_aggregate_groups_by_feature() {
        let logs = vec![
            test_log("qcm", "gpt-4", 0.01),
            test_log("qcm", "gpt-4", 0.02),
            test_log("flashcard", "gpt-4", 0.05),
        ];
        let stats = aggregate_usage_stats(&logs);
        assert_eq!(stats.by_feature.len(), 2);
    }

    #[test]
    fn test_aggregate_sorts_features_by_cost_desc() {
        let logs = vec![
            test_log("qcm", "gpt-4", 0.01),
            test_log("flashcard", "gpt-4", 0.05),
        ];
        let stats = aggregate_usage_stats(&logs);
        assert_eq!(stats.by_feature[0].key, "flashcard");
    }

    #[test]
    fn test_aggregate_sorts_models_by_count_desc() {
        let logs = vec![
            test_log("qcm", "gpt-4", 0.01),
            test_log("qcm", "gpt-4", 0.01),
            test_log("qcm", "gpt-3.5", 0.01),
        ];
        let stats = aggregate_usage_stats(&logs);
        assert_eq!(stats.by_model[0].key, "gpt-4");
    }
}
