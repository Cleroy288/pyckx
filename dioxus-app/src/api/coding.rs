use serde::{Deserialize, Serialize};

use super::endpoints;
use super::helpers::post_json;

#[derive(Debug, Serialize)]
pub struct GenerateCodingRequest {
    pub language: String,
    pub subject: Option<String>,
    pub level: String,
    pub langue: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CodingExerciseResponse {
    pub subject: String,
    pub code_snippet: String,
}

#[derive(Debug, Serialize)]
pub struct CheckCodingRequest {
    pub expected: String,
    pub user_code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CheckCodingResponse {
    pub is_correct: bool,
}

pub async fn generate_coding_game(
    req: &GenerateCodingRequest,
) -> Result<CodingExerciseResponse, String> {
    post_json(endpoints::CODING_GENERATE, req).await
}

pub async fn check_coding_game(
    req: &CheckCodingRequest,
) -> Result<CheckCodingResponse, String> {
    post_json(endpoints::CODING_CHECK, req).await
}
