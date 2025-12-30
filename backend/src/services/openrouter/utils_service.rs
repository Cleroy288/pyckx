//! Utility functions for OpenRouter responses

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

/// Sanitize AI-generated JSON for common errors
///
/// Handles:
/// - Trailing commas before `}` or `]`
/// - Extra whitespace cleanup
/// - Duplicate key detection (logs warning but doesn't modify - too complex for UTF-8 safety)
///
/// This function is UTF-8 safe and will not panic on multi-byte characters.
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
    
    // Phase 2: Check for potential duplicate keys (warning only, no modification)
    // Use char_indices for UTF-8 safety
    let mut in_string = false;
    let mut last_key: Option<String> = None;
    let mut current_key = String::new();
    let mut collecting_key = false;
    let mut seen_keys_at_depth: Vec<std::collections::HashSet<String>> = vec![std::collections::HashSet::new()];
    
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
                // After a colon, the last_key was indeed a key
                if let Some(ref key) = last_key {
                    if let Some(keys) = seen_keys_at_depth.last_mut() {
                        if keys.contains(key) {
                            warn!(
                                key = %key,
                                "Detected duplicate JSON key in AI response"
                            );
                        } else {
                            keys.insert(key.clone());
                        }
                    }
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
/// e.g., `"wrong_answers": "[item1, item2]"` → `"wrong_answers": ["item1", "item2"]`
///
/// This handles the common AI mistake of outputting stringified arrays.
pub fn fix_stringified_arrays(json: &str) -> String {
    use regex::Regex;
    
    // Pattern matches: "field": "[...]" where the brackets are inside a string
    // We look for: "fieldname": "[something]" 
    // And convert to: "fieldname": [something]
    
    // Build regex to find stringified arrays
    // Pattern: "field_name"\s*:\s*"\[([^\]]*)\]"
    let pattern = match Regex::new(r#"("[\w_]+")\s*:\s*"\[([^\]]*)\]""#) {
        Ok(re) => re,
        Err(_) => return json.to_string(), // Regex failed, return unchanged
    };
    
    let result = pattern.replace_all(json, |caps: &regex::Captures| {
        let field_name = &caps[1];
        let array_content = &caps[2];
        
        // Check if the content looks like it should be a proper array
        // Skip if it contains type hints like "string" or "bool"
        let content_trimmed = array_content.trim();
        
        // If content is type hints, convert to placeholder strings
        if content_trimmed.contains("string") 
            || content_trimmed.contains("bool") 
            || content_trimmed.contains("number")
            || content_trimmed.is_empty() 
        {
            // AI outputted type hints - replace with placeholder
            warn!(
                field = %field_name,
                content = %content_trimmed,
                "Fixed stringified array with type hints in AI response"
            );
            format!("{}: [\"Option A\", \"Option B\", \"Option C\"]", field_name)
        } else if content_trimmed.starts_with('"') {
            // Already has quotes inside, just remove outer quotes
            format!("{}: [{}]", field_name, array_content)
        } else {
            // Raw values without quotes - add quotes to each item
            let items: Vec<&str> = array_content.split(',').collect();
            let quoted: Vec<String> = items
                .iter()
                .map(|s| format!("\"{}\"", s.trim()))
                .collect();
            format!("{}: [{}]", field_name, quoted.join(", "))
        }
    });
    
    result.to_string()
}

/// Comprehensive JSON sanitization for AI responses
///
/// Calls multiple sanitization functions in order:
/// 1. fix_stringified_arrays - Fix arrays that are strings
/// 2. sanitize_json_duplicates - Fix trailing commas
pub fn sanitize_ai_json(json: &str) -> String {
    let step1 = fix_stringified_arrays(json);
    sanitize_json_duplicates(&step1)
}
