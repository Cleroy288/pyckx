//! Serde Utilities for AI Response Parsing
//!
//! Provides polymorphic deserializers to handle non-deterministic AI output formats.

use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Deserializes a field that could be a `String` OR a `Vec<String>`.
///
/// This implements Postel's Law: "Be liberal in what you accept."
/// LLMs non-deterministically output text as either:
/// - A single string: `"Paragraph 1.\n\nParagraph 2."`
/// - An array of strings: `["Paragraph 1.", "Paragraph 2."]`
///
/// This function handles both cases by joining arrays with double newlines.
///
/// # Example
/// ```ignore
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(deserialize_with = "deserialize_string_or_vec")]
///     content: String,
/// }
/// ```
pub fn deserialize_string_or_vec<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        // Standard case: already a string
        Value::String(s) => Ok(s),

        // LLM split text into array - join with markdown paragraph breaks
        Value::Array(arr) => {
            let joined = arr
                .iter()
                .filter_map(|val| val.as_str())
                .collect::<Vec<&str>>()
                .join("\n\n");
            Ok(joined)
        }

        // Graceful degradation: stringify numbers and bools
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),

        // Null becomes empty string
        Value::Null => Ok(String::new()),

        // Objects are unexpected but stringify for debugging
        Value::Object(_) => Err(serde::de::Error::custom(
            "Expected String or Array of Strings, got Object",
        )),
    }
}
