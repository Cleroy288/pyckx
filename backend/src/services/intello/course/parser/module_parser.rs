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
        .ok_or_else(|| IntelloError::validation("structure", "Missing or invalid modules array"))?;

    let mut parsed_modules = Vec::new();

    for (idx, module_val) in modules_array.iter().enumerate() {
        match parse_module(module_val, idx) {
            Ok(module) => parsed_modules.push(module),
            Err(e) => {
                warn!(
                    module_index = idx,
                    error = %e,
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
pub fn parse_module(value: &Value, index: usize) -> Result<CourseModule, IntelloError> {
    let title = value
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Untitled Module")
        .to_string();

    let blocks_array = value
        .get("blocks")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            IntelloError::validation("module", format!("Module {} missing blocks array", index))
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
            Err(e) => {
                warn!(
                    module = index,
                    block = block_idx,
                    error = %e,
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
