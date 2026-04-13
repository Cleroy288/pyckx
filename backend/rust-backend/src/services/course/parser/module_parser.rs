#![allow(dead_code)]

use serde_json::Value;
use tracing::warn;

use crate::http_api::data_transfer_object::course::{
    ContentBlock, CourseModule,
};
use crate::services::error_domain::StudyError;

use super::block_parsers::dispatch_block_parser;

/// Default title when module has no title field
const DEFAULT_MODULE_TITLE: &str = "Untitled Module";

/// Parse all modules (partial success - skips malformed)
pub fn parse_modules(
    root: &Value,
) -> Result<Vec<CourseModule>, StudyError> {
    let modules_array = root
        .get("modules")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            StudyError::validation(
                "structure",
                "Missing or invalid modules array",
            )
        })?;

    let parsed: Vec<CourseModule> = modules_array
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| {
            parse_module(val, idx)
                .map_err(|err| {
                    warn!(
                        module_index = idx,
                        error = %err,
                        "Skipping malformed module"
                    );
                    err
                })
                .ok()
        })
        .collect();

    if parsed.is_empty() {
        return Err(StudyError::validation(
            "modules",
            "No valid modules parsed",
        ));
    }
    Ok(parsed)
}

/// Parse a single module from JSON value
pub fn parse_module(
    value: &Value,
    index: usize,
) -> Result<CourseModule, StudyError> {
    let title = value
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_MODULE_TITLE)
        .to_string();

    let blocks_array = value
        .get("blocks")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            StudyError::validation(
                "module",
                format!(
                    "Module {} missing blocks",
                    index
                ),
            )
        })?;

    let blocks = parse_blocks(blocks_array, index);

    Ok(CourseModule { title, blocks })
}

/// Parse blocks from a JSON array, skip invalid ones
fn parse_blocks(
    blocks_array: &[Value],
    module_index: usize,
) -> Vec<ContentBlock> {
    blocks_array
        .iter()
        .enumerate()
        .filter_map(|(idx, val)| {
            match dispatch_block_parser(val) {
                Ok(Some(block)) => Some(block),
                Ok(None) => {
                    warn!(
                        module = module_index,
                        block = idx,
                        "Skipped unknown block type"
                    );
                    None
                }
                Err(err) => {
                    warn!(
                        module = module_index,
                        block = idx,
                        error = %err,
                        "Skipped bad block"
                    );
                    None
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_modules_valid_returns_modules() {
        let root = json!({
            "modules": [{
                "title": "Module 1",
                "blocks": [
                    {"type": "text", "content": "Hello"}
                ]
            }]
        });
        let result = parse_modules(&root);
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_parse_modules_missing_key_returns_error() {
        let root = json!({"other": "data"});
        let result = parse_modules(&root);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_modules_empty_returns_error() {
        let root = json!({"modules": []});
        let result = parse_modules(&root);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_module_valid_returns_module() {
        let val = json!({
            "title": "Intro",
            "blocks": [
                {"type": "title", "content": "Welcome"}
            ]
        });
        let result = parse_module(&val, 0);
        assert_eq!(result.unwrap().title, "Intro");
    }

    #[test]
    fn test_parse_module_missing_blocks_returns_error() {
        let val = json!({"title": "No blocks"});
        let result = parse_module(&val, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_module_missing_title_uses_default() {
        let val = json!({
            "blocks": [
                {"type": "text", "content": "data"}
            ]
        });
        let result = parse_module(&val, 0);
        assert_eq!(
            result.unwrap().title,
            DEFAULT_MODULE_TITLE
        );
    }
}
