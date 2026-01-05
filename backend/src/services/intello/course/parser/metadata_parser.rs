use serde_json::Value;

use crate::http_api::data_transfer_object::intello::course::{CourseMetadata, CourseModule};
use crate::services::intello::error_domain::IntelloError;

use super::module_parser::parse_module;

/// Parse course metadata
pub fn parse_metadata(root: &Value) -> Result<CourseMetadata, IntelloError> {
    let metadata_val = root
        .get("course_metadata")
        .ok_or_else(|| IntelloError::validation("structure", "Missing course_metadata"))?;

    serde_json::from_value(metadata_val.clone())
        .map_err(|e| IntelloError::validation("metadata_parse", e.to_string()))
}

/// Parse synthesis module (reuses module structure)
pub fn parse_synthesis(root: &Value) -> Result<CourseModule, IntelloError> {
    let synthesis_val = root
        .get("synthesis")
        .ok_or_else(|| IntelloError::validation("structure", "Missing synthesis"))?;

    parse_module(synthesis_val, 999) // Use 999 as synthetic index
}
