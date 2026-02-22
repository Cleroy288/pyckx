#![allow(dead_code)]

use serde_json::Value;

use crate::http_api::data_transfer_object::intello::course::{
    CourseMetadata, CourseModule,
};
use crate::services::intello::error_domain::IntelloError;

use super::module_parser::parse_module;

/// Parse course metadata
pub fn parse_metadata(root: &Value) -> Result<CourseMetadata, IntelloError> {
    let metadata_val = root.get("course_metadata").ok_or_else(|| {
        IntelloError::validation("structure", "Missing course_metadata")
    })?;

    serde_json::from_value(metadata_val.clone()).map_err(|err| {
        IntelloError::validation("metadata_parse", err.to_string())
    })
}

/// Parse synthesis module (reuses module structure)
pub fn parse_synthesis(root: &Value) -> Result<CourseModule, IntelloError> {
    let synthesis_val = root.get("synthesis").ok_or_else(|| {
        IntelloError::validation("structure", "Missing synthesis")
    })?;

    parse_module(synthesis_val, 999) // Use 999 as synthetic index
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_metadata_valid_returns_metadata() {
        // arrange
        let root = json!({
            "course_metadata": {
                "title": "Rust Basics",
                "description": "Learn Rust",
                "level": "beginner"
            }
        });

        // act
        let result = parse_metadata(&root);

        // assert
        assert!(result.is_ok());
        let meta = result.unwrap();
        assert_eq!(meta.title, "Rust Basics");
    }

    #[test]
    fn test_parse_metadata_missing_key_returns_error() {
        // arrange
        let root = json!({"other": "data"});

        // act
        let result = parse_metadata(&root);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_synthesis_valid_returns_module() {
        // arrange
        let root = json!({
            "synthesis": {
                "title": "Summary",
                "blocks": [
                    {"type": "text", "content": "Review"}
                ]
            }
        });

        // act
        let result = parse_synthesis(&root);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Summary");
    }

    #[test]
    fn test_parse_synthesis_missing_key_returns_error() {
        // arrange
        let root = json!({"modules": []});

        // act
        let result = parse_synthesis(&root);

        // assert
        assert!(result.is_err());
    }
}
