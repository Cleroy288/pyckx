//! Coding API — HTTP calls to backend coding endpoints.

use serde::{Deserialize, Serialize};

use crate::api::endpoints;
use crate::api::helpers::post_json;

/// Request body for generating a coding exercise.
#[derive(Debug, Serialize)]
pub struct GenerateRequest {
    pub language: String,
    pub subject: Option<String>,
    pub level: String,
    pub langue: Option<String>,
}

/// Server response describing a generated exercise.
#[derive(Debug, Clone, Deserialize)]
pub struct ExerciseResponse {
    pub subject: String,
    pub code_snippet: String,
}

/// Request body for checking a user's solution.
#[derive(Debug, Serialize)]
pub struct CheckRequest {
    pub expected: String,
    pub user_code: String,
}

/// Server response with the verdict on the user's code.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckResponse {
    pub is_correct: bool,
}

/// Generate a new coding exercise from the backend.
pub async fn generate(
    req: &GenerateRequest,
) -> Result<ExerciseResponse, String> {
    post_json(endpoints::CODING_GENERATE, req).await
}

/// Submit user code for evaluation against the expected solution.
pub async fn check(
    req: &CheckRequest,
) -> Result<CheckResponse, String> {
    post_json(endpoints::CODING_CHECK, req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_request_serializes_subject_some() {
        let req = GenerateRequest {
            language: "rust".into(),
            subject: Some("loops".into()),
            level: "easy".into(),
            langue: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"subject\":\"loops\""));
    }

    #[test]
    fn test_generate_request_serializes_subject_none() {
        let req = GenerateRequest {
            language: "rust".into(),
            subject: None,
            level: "easy".into(),
            langue: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"subject\":null"));
    }
}
