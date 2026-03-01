#![allow(dead_code)]

use serde_json::Value;
use tracing::warn;

use crate::http_api::data_transfer_object::intello::course::CourseModule;
use crate::services::intello::error_domain::IntelloError;

use super::block_parsers::dispatch_block_parser;

/// Parse all modules (partial success - skips malformed modules)
pub fn parse_modules(root: &Value) -> Result<Vec<CourseModule>, IntelloError> {
    let modules_array = root
        .get("modules")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            IntelloError::validation(
                "structure",
                "Missing or invalid modules array",
            )
        })?;

    let mut parsed_modules = Vec::new();

    for (idx, module_val) in modules_array.iter().enumerate() {
        match parse_module(module_val, idx) {
            Ok(module) => parsed_modules.push(module),
            Err(err) => {
                warn!(
                    module_index = idx,
                    error = %err,
                    "Failed to parse module, skipping"
                );
                // Continue parsing other modules
            }
        }
    }

    if parsed_modules.is_empty() {
        return Err(IntelloError::validation(
            "modules",
            "No valid modules parsed",
        ));
    }

    Ok(parsed_modules)
}

/// Parse a single module
pub fn parse_module(
    value: &Value,
    index: usize,
) -> Result<CourseModule, IntelloError> {
    let title = value
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Untitled Module")
        .to_string();

    let blocks_array = value
        .get("blocks")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            IntelloError::validation(
                "module",
                format!("Module {} missing blocks array", index),
            )
        })?;

    let mut parsed_blocks = Vec::new();

    for (block_idx, block_val) in blocks_array.iter().enumerate() {
        match dispatch_block_parser(block_val) {
            Ok(Some(block)) => parsed_blocks.push(block),
            Ok(None) => {
                warn!(
                    module = index,
                    block = block_idx,
                    "Skipped unknown block type"
                );
            }
            Err(err) => {
                warn!(
                    module = index,
                    block = block_idx,
                    error = %err,
                    "Failed to parse block, skipping"
                );
            }
        }
    }

    Ok(CourseModule {
        title,
        blocks: parsed_blocks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // -- parse_modules tests --

    #[test]
    fn test_parse_modules_valid_returns_modules() {
        // arrange
        let root = json!({
            "modules": [
                {
                    "title": "Module 1",
                    "blocks": [
                        {"type": "text", "content": "Hello"}
                    ]
                }
            ]
        });

        // act
        let result = parse_modules(&root);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_parse_modules_missing_key_returns_error() {
        // arrange
        let root = json!({"other": "data"});

        // act
        let result = parse_modules(&root);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_modules_empty_array_returns_error() {
        // arrange
        let root = json!({"modules": []});

        // act
        let result = parse_modules(&root);

        // assert - no valid modules parsed
        assert!(result.is_err());
    }

    // -- parse_module tests --

    #[test]
    fn test_parse_module_valid_returns_module() {
        // arrange
        let val = json!({
            "title": "Intro",
            "blocks": [
                {"type": "title", "content": "Welcome"}
            ]
        });

        // act
        let result = parse_module(&val, 0);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Intro");
    }

    #[test]
    fn test_parse_module_missing_blocks_returns_error() {
        // arrange
        let val = json!({"title": "No blocks"});

        // act
        let result = parse_module(&val, 0);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_module_missing_title_uses_default() {
        // arrange
        let val = json!({
            "blocks": [
                {"type": "text", "content": "data"}
            ]
        });

        // act
        let result = parse_module(&val, 0);

        // assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Untitled Module");
    }
}
