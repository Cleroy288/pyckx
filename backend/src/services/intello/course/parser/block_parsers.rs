// Block parsers for AI-generated content. This module contains parsing functions.

use serde_json::Value;

use crate::http_api::data_transfer_object::intello::course::*;
use crate::services::intello::error_domain::IntelloError;

/// Route to appropriate block parser based on "type" field
pub fn dispatch_block_parser(block: &Value) -> Result<Option<ContentBlock>, IntelloError> {
    let type_str = block.get("type").and_then(|v| v.as_str()).unwrap_or("");

    match type_str {
        "title" => parse_title(block).map(Some),
        "subtitle" => parse_subtitle(block).map(Some),
        "text" => parse_text_section(block).map(Some),
        "schema" => parse_schema(block).map(Some),
        "qcm_set" => parse_qcm_set(block).map(Some),
        "flashcard_set" => parse_flashcard_set(block).map(Some),
        "true_false_set" => parse_true_false_set(block).map(Some),
        _ => {
            tracing::debug!(unknown_type = type_str, "Unknown block type");
            Ok(None) // Skip unknown blocks
        }
    }
}

/// Parse title block
fn parse_title(value: &Value) -> Result<ContentBlock, IntelloError> {
    let content = value
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| IntelloError::validation("title", "Missing content"))?
        .to_string();

    Ok(ContentBlock::Title { content })
}

/// Parse subtitle block
fn parse_subtitle(value: &Value) -> Result<ContentBlock, IntelloError> {
    let content = value
        .get("content")
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
