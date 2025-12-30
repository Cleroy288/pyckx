//! Resilient Course Parser
//!
//! Implements Postel's Law: "Be liberal in what you accept."
//! 
//! This parser handles non-deterministic AI output by:
//! - Parsing incrementally using serde_json::Value
//! - Routing to dedicated parsers per block type
//! - Skipping malformed blocks instead of failing entire course
//! - Logging warnings for debugging

use serde_json::Value;
use tracing::{debug, warn, instrument};

use crate::http_api::data_transfer_object::intello::course::*;
use crate::services::intello::error_domain::IntelloError;
use crate::services::openrouter::utils_service::{extract_json_from_response, sanitize_ai_json};

// == COURSE PARSER ==

/// Resilient parser for AI-generated course JSON
pub struct CourseParser;

impl CourseParser {
    /// Parse entire course from raw JSON string
    /// 
    /// This is the entry point that:
    /// 1. Sanitizes the JSON (fixes common AI errors)
    /// 2. Parses metadata
    /// 3. Parses modules (each module can partially fail)
    /// 4. Parses synthesis module
    #[instrument(skip(raw_json), fields(json_len = raw_json.len()))]
    pub fn parse_course(raw_json: &str) -> Result<GeneratedCourse, IntelloError> {
        // Step 1: Extract and sanitize JSON
        let extracted = extract_json_from_response(raw_json);
        let sanitized = sanitize_ai_json(&extracted);
        
        debug!(sanitized_len = sanitized.len(), "Sanitized JSON");
        
        // Step 2: Parse into generic Value to avoid immediate failure
        let root: Value = serde_json::from_str(&sanitized)
            .map_err(|e| IntelloError::validation("json_parse", 
                format!("Failed to parse JSON: {}", e)))?;
        
        // Step 3: Extract and parse metadata
        let metadata = Self::parse_metadata(&root)?;
        
        // Step 4: Parse modules array
        let modules = Self::parse_modules(&root)?;
        
        // Step 5: Parse synthesis module
        let synthesis = Self::parse_synthesis(&root)?;
        
        Ok(GeneratedCourse {
            course_metadata: metadata,
            modules,
            synthesis,
        })
    }
    
    /// Parse course metadata
    fn parse_metadata(root: &Value) -> Result<CourseMetadata, IntelloError> {
        let metadata_val = root.get("course_metadata")
            .ok_or_else(|| IntelloError::validation("structure", "Missing course_metadata"))?;
        
        serde_json::from_value(metadata_val.clone())
            .map_err(|e| IntelloError::validation("metadata_parse", e.to_string()))
    }
    
    /// Parse all modules (partial success - skips malformed modules)
    fn parse_modules(root: &Value) -> Result<Vec<CourseModule>, IntelloError> {
        let modules_array = root.get("modules")
            .and_then(|v| v.as_array())
            .ok_or_else(|| IntelloError::validation("structure", "Missing or invalid modules array"))?;
        
        let mut parsed_modules = Vec::new();
        
        for (idx, module_val) in modules_array.iter().enumerate() {
            match Self::parse_module(module_val, idx) {
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
            return Err(IntelloError::validation("modules", "No valid modules parsed"));
        }
        
        Ok(parsed_modules)
    }
    
    /// Parse synthesis module (reuses module structure)
    fn parse_synthesis(root: &Value) -> Result<CourseModule, IntelloError> {
        let synthesis_val = root.get("synthesis")
            .ok_or_else(|| IntelloError::validation("structure", "Missing synthesis"))?;
        
        Self::parse_module(synthesis_val, 999) // Use 999 as synthetic index
    }
    
    /// Parse a single module
    fn parse_module(value: &Value, index: usize) -> Result<CourseModule, IntelloError> {
        let title = value.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled Module")
            .to_string();
        
        let blocks_array = value.get("blocks")
            .and_then(|v| v.as_array())
            .ok_or_else(|| IntelloError::validation("module", 
                format!("Module {} missing blocks array", index)))?;
        
        let mut parsed_blocks = Vec::new();
        
        for (block_idx, block_val) in blocks_array.iter().enumerate() {
            match Self::dispatch_block_parser(block_val) {
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
    
    // == BLOCK ROUTING ==
    
    /// Route to appropriate block parser based on "type" field
    fn dispatch_block_parser(block: &Value) -> Result<Option<ContentBlock>, IntelloError> {
        let type_str = block.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match type_str {
            "title" => Self::parse_title(block).map(Some),
            "subtitle" => Self::parse_subtitle(block).map(Some),
            "text" => Self::parse_text_section(block).map(Some),
            "schema" => Self::parse_schema(block).map(Some),
            "qcm_set" => Self::parse_qcm_set(block).map(Some),
            "flashcard_set" => Self::parse_flashcard_set(block).map(Some),
            "true_false_set" => Self::parse_true_false_set(block).map(Some),
            _ => {
                debug!(unknown_type = type_str, "Unknown block type");
                Ok(None) // Skip unknown blocks
            }
        }
    }
    
    // == INDIVIDUAL BLOCK PARSERS ==
    
    /// Parse title block
    fn parse_title(value: &Value) -> Result<ContentBlock, IntelloError> {
        let content = value.get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| IntelloError::validation("title", "Missing content"))?
            .to_string();
        
        Ok(ContentBlock::Title { content })
    }
    
    /// Parse subtitle block
    fn parse_subtitle(value: &Value) -> Result<ContentBlock, IntelloError> {
        let content = value.get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| IntelloError::validation("subtitle", "Missing content"))?
            .to_string();
        
        Ok(ContentBlock::Subtitle { content })
    }
    
    /// Parse text section block
    fn parse_text_section(value: &Value) -> Result<ContentBlock, IntelloError> {
        // Use serde to trigger deserialize_string_or_vec helper
        let block: ContentBlock = serde_json::from_value(value.clone())
            .map_err(|e| IntelloError::validation("text_section", e.to_string()))?;
        
        Ok(block)
    }
    
    /// Parse schema/diagram block
    fn parse_schema(value: &Value) -> Result<ContentBlock, IntelloError> {
        let block: ContentBlock = serde_json::from_value(value.clone())
            .map_err(|e| IntelloError::validation("schema", e.to_string()))?;
        
        // Note: Mermaid sanitization happens at render time
        Ok(block)
    }
    
    /// Parse QCM set block
    fn parse_qcm_set(value: &Value) -> Result<ContentBlock, IntelloError> {
        // Handle both {"type": "qcm_set", "data": {...}} and direct data
        let data_val = value.get("data").unwrap_or(value);
        
        let data: QcmSetPayload = serde_json::from_value(data_val.clone())
            .map_err(|e| IntelloError::validation("qcm_set", e.to_string()))?;
        
        Ok(ContentBlock::QcmSet { data })
    }
    
    /// Parse flashcard set block
    fn parse_flashcard_set(value: &Value) -> Result<ContentBlock, IntelloError> {
        let data_val = value.get("data").unwrap_or(value);
        
        let data: FlashcardSetPayload = serde_json::from_value(data_val.clone())
            .map_err(|e| IntelloError::validation("flashcard_set", e.to_string()))?;
        
        Ok(ContentBlock::FlashcardSet { data })
    }
    
    /// Parse true/false set block
    fn parse_true_false_set(value: &Value) -> Result<ContentBlock, IntelloError> {
        let data_val = value.get("data").unwrap_or(value);
        
        let data: TrueFalseSetPayload = serde_json::from_value(data_val.clone())
            .map_err(|e| IntelloError::validation("true_false_set", e.to_string()))?;
        
        Ok(ContentBlock::TrueFalseSet { data })
    }
}
