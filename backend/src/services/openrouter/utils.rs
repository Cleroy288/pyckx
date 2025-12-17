//! Utility functions for OpenRouter responses

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
