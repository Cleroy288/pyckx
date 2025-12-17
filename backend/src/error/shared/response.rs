//! Error response - JSON structure for API errors

use serde::Serialize;

/// JSON response body for errors
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: &'static str,
    pub message: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<&'static str>,
}
