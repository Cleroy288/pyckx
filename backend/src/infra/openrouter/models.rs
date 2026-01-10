//! AI Model Registry for OpenRouter

use serde::{Deserialize, Serialize};

// ============================================================
// MODEL DEFINITION
// ============================================================

/// Complete model definition with pricing
#[derive(Debug, Clone, Copy)]
pub struct ModelDefinition {
    /// Model identifier (e.g., "google/gemini-3-flash-preview")
    pub id: &'static str,
    /// Human-readable display name (for future admin features)
    #[allow(dead_code)]
    pub display_name: &'static str,
    /// Cost per million input tokens in USD
    pub input_cost_per_million: f64,
    /// Cost per million output tokens in USD
    pub output_cost_per_million: f64,
    /// Whether this is a free model (for future filtering)
    #[allow(dead_code)]
    pub is_free: bool,
}

// ============================================================
// CENTRALIZED MODEL REGISTRY
// ============================================================

/// All available AI models with pricing
pub const MODELS: &[ModelDefinition] = &[ModelDefinition {
    id: "google/gemini-3-flash-preview",
    display_name: "Gemini 3 Flash Preview",
    input_cost_per_million: 0.50,
    output_cost_per_million: 3.00,
    is_free: false,
}];

/// Default model for ALL operations (course generation, games, etc.)
pub const DEFAULT_MODEL: &str = "google/gemini-3-flash-preview";

// ============================================================
// HELPER FUNCTIONS
// ============================================================

/// Get a model definition by ID
pub fn get_model(id: &str) -> Option<&'static ModelDefinition> {
    MODELS.iter().find(|m| m.id == id)
}

// ============================================================
// COST CALCULATION
// ============================================================

/// Result of cost calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostResult {
    pub model_id: String,
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub input_cost_usd: f64,
    pub output_cost_usd: f64,
    pub total_cost_usd: f64,
}

/// Calculate cost for an AI request
pub fn calculate_cost(model_id: &str, input_tokens: u32, output_tokens: u32) -> CostResult {
    let model = get_model(model_id);

    let (input_cost, output_cost) = match model {
        Some(m) => {
            let input = (input_tokens as f64 / 1_000_000.0) * m.input_cost_per_million;
            let output = (output_tokens as f64 / 1_000_000.0) * m.output_cost_per_million;
            (input, output)
        }
        None => (0.0, 0.0), // Unknown model, assume free
    };

    CostResult {
        model_id: model_id.to_string(),
        input_tokens,
        output_tokens,
        input_cost_usd: input_cost,
        output_cost_usd: output_cost,
        total_cost_usd: input_cost + output_cost,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model() {
        let model = get_model("google/gemini-3-flash-preview");
        assert!(model.is_some());
        assert_eq!(model.unwrap().input_cost_per_million, 0.50);
    }

    #[test]
    fn test_calculate_cost() {
        let cost = calculate_cost("google/gemini-3-flash-preview", 1_000_000, 1_000_000);
        assert!((cost.input_cost_usd - 0.50).abs() < 0.001);
        assert!((cost.output_cost_usd - 3.00).abs() < 0.001);
        assert!((cost.total_cost_usd - 3.50).abs() < 0.001);
    }
}
