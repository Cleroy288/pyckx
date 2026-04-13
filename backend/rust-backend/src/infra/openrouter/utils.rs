//! Utility functions for OpenRouter responses

use std::collections::HashSet;

use tracing::warn;

/// Extract content from a generic markdown code block.
/// Returns text between the opening ``` line and closing ```.
fn extract_from_code_block(
    text: &str,
) -> Option<String> {
    let start = text.find("```")?;
    let after_marker = &text[start + 3..];
    let newline = after_marker.find('\n')?;
    let content = &after_marker[newline + 1..];
    let end = content.find("```")?;
    Some(content[..end].trim().to_string())
}

/// Extract JSON from AI response (handles markdown
/// code blocks and embedded JSON objects)
pub fn extract_json_from_response(
    response: &str,
) -> String {
    let trimmed = response.trim();

    if trimmed.starts_with("```json") {
        if let Some(s) = trimmed.find("```json") {
            let after = &trimmed[s + 7..];
            if let Some(end) = after.find("```") {
                return after[..end].trim().to_string();
            }
        }
    } else if trimmed.starts_with("```") {
        if let Some(v) = extract_from_code_block(trimmed)
        {
            return v;
        }
    }

    if let Some(s) = trimmed.find('{') {
        if let Some(e) = trimmed.rfind('}') {
            return trimmed[s..=e].to_string();
        }
    }

    trimmed.to_string()
}

/// Remove trailing commas before `}` or `]`
fn remove_trailing_commas(json: &str) -> String {
    json.replace(",}", "}")
        .replace(",]", "]")
        .replace(", }", "}")
        .replace(", ]", "]")
        .replace(",\n}", "\n}")
        .replace(",\n]", "\n]")
        .replace(",\r\n}", "\r\n}")
        .replace(",\r\n]", "\r\n]")
        .replace(",  }", "}")
        .replace(",  ]", "]")
}

/// Log warnings for duplicate JSON keys at each depth
fn detect_duplicate_keys(json: &str) {
    let mut in_string = false;
    let mut last_key: Option<String> = None;
    let mut current_key = String::new();
    let mut collecting = false;
    let mut seen: Vec<HashSet<String>> =
        vec![HashSet::new()];

    for ch in json.chars() {
        match ch {
            '"' if !in_string => {
                in_string = true;
                collecting = true;
                current_key.clear();
            }
            '"' if in_string => {
                in_string = false;
                if collecting {
                    last_key = Some(current_key.clone());
                    collecting = false;
                }
            }
            ':' if !in_string => {
                if let Some(ref k) = last_key {
                    check_duplicate(k, &mut seen);
                }
                last_key = None;
            }
            '{' if !in_string => {
                seen.push(HashSet::new());
            }
            '}' if !in_string => {
                seen.pop();
            }
            _ if in_string && collecting => {
                current_key.push(ch);
            }
            _ => {}
        }
    }
}

/// Check if key is duplicate at current depth
fn check_duplicate(
    key: &str,
    seen: &mut [HashSet<String>],
) {
    if let Some(keys) = seen.last_mut() {
        if keys.contains(key) {
            warn!(
                key = %key,
                "Duplicate JSON key in AI response"
            );
        } else {
            keys.insert(key.to_string());
        }
    }
}

/// Sanitize AI-generated JSON: remove trailing commas
/// and warn about duplicate keys
pub fn sanitize_json_duplicates(json: &str) -> String {
    let cleaned = remove_trailing_commas(json);
    detect_duplicate_keys(&cleaned);
    cleaned
}

/// Fix arrays accidentally serialized as strings
pub fn fix_stringified_arrays(json: &str) -> String {
    use regex::Regex;

    let re = match Regex::new(
        r#"("[\w_]+")\s*:\s*"\[([^\]]*)\]""#,
    ) {
        Ok(re) => re,
        Err(_) => return json.to_string(),
    };

    let result =
        re.replace_all(json, |caps: &regex::Captures| {
            format_array_fix(&caps[1], &caps[2])
        });
    result.to_string()
}

/// Format replacement for a stringified array match
fn format_array_fix(
    field_name: &str,
    array_content: &str,
) -> String {
    let trimmed = array_content.trim();

    if trimmed.contains("string")
        || trimmed.contains("bool")
        || trimmed.contains("number")
        || trimmed.is_empty()
    {
        warn!(
            field = %field_name,
            content = %trimmed,
            "Fixed stringified array with type hints"
        );
        return format!(
            "{}: [\"Option A\", \"Option B\", \"Option C\"]",
            field_name
        );
    }

    if trimmed.starts_with('"') {
        return format!(
            "{}: [{}]",
            field_name, array_content
        );
    }

    let items: Vec<&str> =
        array_content.split(',').collect();
    let quoted: Vec<String> = items
        .iter()
        .map(|s| format!("\"{}\"", s.trim()))
        .collect();
    format!("{}: [{}]", field_name, quoted.join(", "))
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
