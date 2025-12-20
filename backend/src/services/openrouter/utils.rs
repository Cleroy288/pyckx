//! Utility functions for OpenRouter responses

use std::collections::HashSet;
use tracing::warn;

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
        if let Some(start) = trimmed.find("```") {
            let after_marker = &trimmed[start + 3..];
            // Skip to newline
            if let Some(newline) = after_marker.find('\n') {
                let content = &after_marker[newline + 1..];
                if let Some(end) = content.find("```") {
                    return content[..end].trim().to_string();
                }
            }
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

// == JSON Sanitization for AI Responses // ==

/// Remove duplicate JSON keys from AI-generated responses
///
/// AI models sometimes generate invalid JSON with duplicate keys like:
/// ```json
/// { "expected_answer": "...", "expected_answer": "..." }
/// ```
///
/// This function detects and removes duplicate occurrences, keeping only the first.
/// Returns the sanitized JSON string and logs a warning if duplicates were found.
pub fn sanitize_json_duplicates(json: &str) -> String {
    
    let mut seen_keys: HashSet<String> = HashSet::new();
    let mut duplicates_found = false;
    let mut result = String::with_capacity(json.len());
    let mut last_end = 0;
    
    
    // Track brace depth to handle nested objects
    let chars: Vec<char> = json.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        // Check for key pattern at current position
        if chars[i] == '"' && i > 0 && (chars[i - 1] == '{' || chars[i - 1] == ',' || chars[i - 1].is_whitespace()) {
            // Find the end of the key
            if let Some(key_end) = json[i + 1..].find('"') {
                let key = &json[i + 1..i + 1 + key_end];
                
                // Check if this key is a duplicate within the same object context
                if seen_keys.contains(key) {
                    duplicates_found = true;
                    warn!(
                        key = %key,
                        "Detected duplicate JSON key in AI response, removing duplicate"
                    );
                    
                    // Skip this key-value pair
                    // Find the value end (next comma or closing brace at same level)
                    let value_start = i + 1 + key_end + 1; // After closing quote
                    if let Some(colon_offset) = json[value_start..].find(':') {
                        let after_colon = value_start + colon_offset + 1;
                        
                        // Find end of value (comma or closing brace)
                        let mut depth = 0;
                        let mut in_string = false;
                        let mut j = after_colon;
                        
                        while j < chars.len() {
                            match chars[j] {
                                '"' if j == 0 || chars[j - 1] != '\\' => in_string = !in_string,
                                '{' | '[' if !in_string => depth += 1,
                                '}' | ']' if !in_string => {
                                    if depth == 0 {
                                        break;
                                    }
                                    depth -= 1;
                                }
                                ',' if !in_string && depth == 0 => {
                                    j += 1; // Include the comma in what we skip
                                    break;
                                }
                                _ => {}
                            }
                            j += 1;
                        }
                        
                        // Skip from current position to end of value
                        // But first, output everything before this duplicate
                        result.push_str(&json[last_end..i]);
                        
                        // Handle leading comma if present
                        let trimmed_result = result.trim_end();
                        if trimmed_result.ends_with(',') {
                            result = trimmed_result.to_string();
                        }
                        
                        last_end = j;
                        i = j;
                        continue;
                    }
                } else {
                    seen_keys.insert(key.to_string());
                }
            }
        }
        
        // Reset seen_keys when entering a new object
        if chars[i] == '{' {
            seen_keys.clear();
        }
        
        i += 1;
    }
    
    // Append remaining content
    result.push_str(&json[last_end..]);
    
    if duplicates_found {
        // Clean up any trailing commas before closing braces
        let cleaned = result
            .replace(",}", "}")
            .replace(", }", "}")
            .replace(",\n}", "\n}");
        return cleaned;
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_json_duplicates_no_duplicates() {
        let json = r#"{"question": "test", "answer": "value"}"#;
        assert_eq!(sanitize_json_duplicates(json), json);
    }

    #[test]
    fn test_sanitize_json_duplicates_with_duplicate() {
        let json = r#"{"question": "test", "answer": "first", "answer": "second"}"#;
        let result = sanitize_json_duplicates(json);
        // Should keep first occurrence only
        assert!(result.contains(r#""answer": "first""#));
        assert!(!result.contains(r#""answer": "second""#));
    }

    #[test]
    fn test_extract_json_from_markdown() {
        let response = "```json\n{\"test\": 1}\n```";
        assert_eq!(extract_json_from_response(response), "{\"test\": 1}");
    }
}
