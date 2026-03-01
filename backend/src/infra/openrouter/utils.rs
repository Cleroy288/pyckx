//! Utility functions for OpenRouter responses

use tracing::warn;

// ============================================================
// JSON EXTRACTION
// ============================================================

/// Extract content from a generic markdown code block
/// Returns the text between the opening ``` line and closing ```
fn extract_from_code_block(text: &str) -> Option<String> {
    let start = text.find("```")?;
    let after_marker = &text[start + 3..];
    let newline = after_marker.find('\n')?;
    let content = &after_marker[newline + 1..];
    let end = content.find("```")?;
    Some(content[..end].trim().to_string())
}

/// Extract JSON from AI response (handles markdown code blocks)
pub fn extract_json_from_response(response: &str) -> String {
    let trimmed = response.trim();

    // Check if response is wrapped in markdown code block
    if trimmed.starts_with("```json") {
        // Extract content between ```json and ```
        if let Some(start) = trimmed.find("```json") {
            let after_marker = &trimmed[start + 7..];
            if let Some(end) = after_marker.find("```") {
                return after_marker[..end].trim().to_string();
            }
        }
    } else if trimmed.starts_with("```") {
        // Generic code block
        if let Some(extracted) = extract_from_code_block(trimmed) {
            return extracted;
        }
    }

    // Try to find JSON object directly
    if let Some(start) = trimmed.find('{') {
        if let Some(end) = trimmed.rfind('}') {
            return trimmed[start..=end].to_string();
        }
    }

    // Return as-is if no extraction needed
    trimmed.to_string()
}

// ============================================================
// JSON SANITIZATION
// ============================================================

/// Check if a JSON key is a duplicate at the current depth
/// Logs a warning if duplicate, otherwise inserts into the set
fn check_duplicate_key(
    seen_keys: &mut [std::collections::HashSet<String>],
    key: &str,
) {
    if let Some(keys) = seen_keys.last_mut() {
        if keys.contains(key) {
            warn!(
                key = %key,
                "Detected duplicate JSON key in AI response"
            );
        } else {
            keys.insert(key.to_string());
        }
    }
}

/// Sanitize AI-generated JSON for common errors
///
/// Handles:
/// - Trailing commas before `}` or `]`
/// - Extra whitespace cleanup
/// - Duplicate key detection (logs warning)
pub fn sanitize_json_duplicates(json: &str) -> String {
    // Phase 1: Clean up trailing commas (most common AI error)
    let cleaned = json
        .replace(",}", "}")
        .replace(",]", "]")
        .replace(", }", "}")
        .replace(", ]", "]")
        .replace(",\n}", "\n}")
        .replace(",\n]", "\n]")
        .replace(",\r\n}", "\r\n}")
        .replace(",\r\n]", "\r\n]")
        .replace(",  }", "}")
        .replace(",  ]", "]");

    // Phase 2: Check for potential duplicate keys (warning only)
    let mut in_string = false;
    let mut last_key: Option<String> = None;
    let mut current_key = String::new();
    let mut collecting_key = false;
    let mut seen_keys_at_depth: Vec<std::collections::HashSet<String>> =
        vec![std::collections::HashSet::new()];

    for (_, ch) in cleaned.char_indices() {
        match ch {
            '"' if !in_string => {
                in_string = true;
                collecting_key = true;
                current_key.clear();
            }
            '"' if in_string => {
                in_string = false;
                if collecting_key {
                    last_key = Some(current_key.clone());
                    collecting_key = false;
                }
            }
            ':' if !in_string => {
                if let Some(ref key) = last_key {
                    check_duplicate_key(&mut seen_keys_at_depth, key);
                }
                last_key = None;
            }
            '{' if !in_string => {
                seen_keys_at_depth.push(std::collections::HashSet::new());
            }
            '}' if !in_string => {
                seen_keys_at_depth.pop();
            }
            _ if in_string && collecting_key => {
                current_key.push(ch);
            }
            _ => {}
        }
    }

    cleaned
}

/// Fix AI error where arrays are accidentally serialized as strings
pub fn fix_stringified_arrays(json: &str) -> String {
    use regex::Regex;

    let pattern = match Regex::new(r#"("[\w_]+")\s*:\s*"\[([^\]]*)\]""#) {
        Ok(re) => re,
        Err(_) => return json.to_string(),
    };

    let result = pattern.replace_all(json, |caps: &regex::Captures| {
        let field_name = &caps[1];
        let array_content = &caps[2];
        let content_trimmed = array_content.trim();

        if content_trimmed.contains("string")
            || content_trimmed.contains("bool")
            || content_trimmed.contains("number")
            || content_trimmed.is_empty()
        {
            warn!(
                field = %field_name,
                content = %content_trimmed,
                "Fixed stringified array with type hints"
            );
            format!(
                "{}: [\"Option A\", \"Option B\", \"Option C\"]",
                field_name
            )
        } else if content_trimmed.starts_with('"') {
            format!("{}: [{}]", field_name, array_content)
        } else {
            let items: Vec<&str> = array_content.split(',').collect();
            let quoted: Vec<String> =
                items.iter().map(|s| format!("\"{}\"", s.trim())).collect();
            format!("{}: [{}]", field_name, quoted.join(", "))
        }
    });

    result.to_string()
}

/// Comprehensive JSON sanitization for AI responses
pub fn sanitize_ai_json(json: &str) -> String {
    let step1 = fix_stringified_arrays(json);
    sanitize_json_duplicates(&step1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- extract_json_from_response tests --

    #[test]
    fn test_extract_json_raw_json_object_returns_as_is() {
        // arrange
        let input = r#"{"key": "value"}"#;

        // act
        let result = extract_json_from_response(input);

        // assert
        assert_eq!(result, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_json_markdown_json_block_extracts_content() {
        // arrange
        let input = "```json\n{\"key\": \"value\"}\n```";

        // act
        let result = extract_json_from_response(input);

        // assert
        assert_eq!(result, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_json_generic_code_block_extracts_content() {
        // arrange
        let input = "```\n{\"key\": \"value\"}\n```";

        // act
        let result = extract_json_from_response(input);

        // assert
        assert_eq!(result, r#"{"key": "value"}"#);
    }

    #[test]
    fn test_extract_json_no_json_returns_trimmed_input() {
        // arrange
        let input = "  just some text  ";

        // act
        let result = extract_json_from_response(input);

        // assert
        assert_eq!(result, "just some text");
    }

    #[test]
    fn test_extract_json_embedded_object_extracts_braces() {
        // arrange
        let input = "Here is the JSON: {\"a\": 1} done";

        // act
        let result = extract_json_from_response(input);

        // assert
        assert_eq!(result, r#"{"a": 1}"#);
    }

    // -- sanitize_json_duplicates tests --

    #[test]
    fn test_sanitize_trailing_comma_before_brace() {
        // arrange
        let input = r#"{"a": 1,}"#;

        // act
        let result = sanitize_json_duplicates(input);

        // assert
        assert_eq!(result, r#"{"a": 1}"#);
    }

    #[test]
    fn test_sanitize_trailing_comma_before_bracket() {
        // arrange
        let input = r#"[1, 2, 3,]"#;

        // act
        let result = sanitize_json_duplicates(input);

        // assert
        assert_eq!(result, r#"[1, 2, 3]"#);
    }

    #[test]
    fn test_sanitize_clean_json_unchanged() {
        // arrange
        let input = r#"{"name": "Alice", "age": 30}"#;

        // act
        let result = sanitize_json_duplicates(input);

        // assert
        assert_eq!(result, input);
    }

    #[test]
    fn test_sanitize_trailing_comma_with_newline() {
        // arrange
        let input = "{\n  \"a\": 1,\n}";

        // act
        let result = sanitize_json_duplicates(input);

        // assert
        assert_eq!(result, "{\n  \"a\": 1\n}");
    }

    // -- fix_stringified_arrays tests --

    #[test]
    fn test_fix_stringified_array_with_quoted_items() {
        // arrange
        let input =
            r#"{"options": "["opt1", "opt2"]"}"#;

        // act
        let result = fix_stringified_arrays(input);

        // assert - should unwrap the stringified array
        assert!(result.contains("["));
        assert!(!result.contains("\"["));
    }

    #[test]
    fn test_fix_normal_array_unchanged() {
        // arrange
        let input = r#"{"items": ["a", "b"]}"#;

        // act
        let result = fix_stringified_arrays(input);

        // assert
        assert_eq!(result, input);
    }

    #[test]
    fn test_fix_empty_input_returns_empty() {
        // arrange / act
        let result = fix_stringified_arrays("");

        // assert
        assert_eq!(result, "");
    }

    // -- sanitize_ai_json tests (composition) --

    #[test]
    fn test_sanitize_ai_json_fixes_trailing_comma() {
        // arrange
        let input = r#"{"a": 1,}"#;

        // act
        let result = sanitize_ai_json(input);

        // assert
        assert_eq!(result, r#"{"a": 1}"#);
    }

    #[test]
    fn test_sanitize_ai_json_clean_input_unchanged() {
        // arrange
        let input = r#"{"key": "value"}"#;

        // act
        let result = sanitize_ai_json(input);

        // assert
        assert_eq!(result, input);
    }
}
