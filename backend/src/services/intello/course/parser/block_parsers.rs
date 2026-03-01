#![allow(dead_code)]
// Block parsers for AI-generated content. This module contains parsing functions.

use serde_json::Value;

use crate::http_api::data_transfer_object::intello::course::*;
use crate::services::intello::error_domain::IntelloError;

/// Route to appropriate block parser based on "type" field
pub fn dispatch_block_parser(
    block: &Value,
) -> Result<Option<ContentBlock>, IntelloError> {
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
    let block: ContentBlock =
        serde_json::from_value(value.clone()).map_err(|err| {
            IntelloError::validation("text_section", err.to_string())
        })?;

    Ok(block)
}

/// Parse schema/diagram block
fn parse_schema(value: &Value) -> Result<ContentBlock, IntelloError> {
    let block: ContentBlock = serde_json::from_value(value.clone())
        .map_err(|err| IntelloError::validation("schema", err.to_string()))?;

    // Note: Mermaid sanitization happens at render time
    Ok(block)
}

/// Parse QCM set block
fn parse_qcm_set(value: &Value) -> Result<ContentBlock, IntelloError> {
    // Handle both {"type": "qcm_set", "data": {...}} and direct data
    let data_val = value.get("data").unwrap_or(value);

    let data: QcmSetPayload = serde_json::from_value(data_val.clone())
        .map_err(|err| IntelloError::validation("qcm_set", err.to_string()))?;

    Ok(ContentBlock::QcmSet { data })
}

/// Parse flashcard set block
fn parse_flashcard_set(value: &Value) -> Result<ContentBlock, IntelloError> {
    let data_val = value.get("data").unwrap_or(value);

    let data: FlashcardSetPayload = serde_json::from_value(data_val.clone())
        .map_err(|err| {
            IntelloError::validation("flashcard_set", err.to_string())
        })?;

    Ok(ContentBlock::FlashcardSet { data })
}

/// Parse true/false set block
fn parse_true_false_set(value: &Value) -> Result<ContentBlock, IntelloError> {
    let data_val = value.get("data").unwrap_or(value);

    let data: TrueFalseSetPayload = serde_json::from_value(data_val.clone())
        .map_err(|err| {
            IntelloError::validation("true_false_set", err.to_string())
        })?;

    Ok(ContentBlock::TrueFalseSet { data })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_dispatch_title_block_returns_title() {
        // arrange
        let block = json!({"type": "title", "content": "Hello"});

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_dispatch_subtitle_block_returns_subtitle() {
        // arrange
        let block = json!({
            "type": "subtitle",
            "content": "Sub"
        });

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_dispatch_text_block_returns_text() {
        // arrange
        let block = json!({"type": "text", "content": "Body"});

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_dispatch_unknown_type_returns_none() {
        // arrange
        let block = json!({"type": "unknown_block"});

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_dispatch_missing_type_returns_none() {
        // arrange
        let block = json!({"content": "no type"});

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_dispatch_title_missing_content_returns_error() {
        // arrange
        let block = json!({"type": "title"});

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_dispatch_schema_block_returns_schema() {
        // arrange
        let block = json!({
            "type": "schema",
            "language": "mermaid",
            "content": "graph TD"
        });

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_dispatch_qcm_set_valid_returns_block() {
        // arrange
        let block = json!({
            "type": "qcm_set",
            "data": {
                "name": "Test QCM",
                "description": "desc",
                "level": "easy",
                "subjects": [],
                "questions": []
            }
        });

        // act
        let result = dispatch_block_parser(&block);

        // assert
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}
