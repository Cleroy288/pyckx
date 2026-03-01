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
pub fn deserialize_string_or_vec<'de, D>(
    deserializer: D,
) -> Result<String, D::Error>
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    /// Test struct using the custom deserializer
    #[derive(Deserialize)]
    struct TestStruct {
        #[serde(deserialize_with = "deserialize_string_or_vec")]
        content: String,
    }

    /// Helper: deserialize JSON into TestStruct
    fn parse(json: &str) -> Result<TestStruct, serde_json::Error> {
        serde_json::from_str(json)
    }

    #[test]
    fn test_deserialize_plain_string() {
        // arrange
        let json = r#"{"content": "hello world"}"#;

        // act
        let result = parse(json).unwrap();

        // assert
        assert_eq!(result.content, "hello world");
    }

    #[test]
    fn test_deserialize_array_of_strings() {
        // arrange
        let json = r#"{"content": ["line 1", "line 2"]}"#;

        // act
        let result = parse(json).unwrap();

        // assert
        assert_eq!(result.content, "line 1\n\nline 2");
    }

    #[test]
    fn test_deserialize_empty_array() {
        // arrange
        let json = r#"{"content": []}"#;

        // act
        let result = parse(json).unwrap();

        // assert
        assert_eq!(result.content, "");
    }

    #[test]
    fn test_deserialize_null_returns_empty() {
        // arrange
        let json = r#"{"content": null}"#;

        // act
        let result = parse(json).unwrap();

        // assert
        assert_eq!(result.content, "");
    }

    #[test]
    fn test_deserialize_number_stringified() {
        // arrange
        let json = r#"{"content": 42}"#;

        // act
        let result = parse(json).unwrap();

        // assert
        assert_eq!(result.content, "42");
    }

    #[test]
    fn test_deserialize_bool_stringified() {
        // arrange
        let json = r#"{"content": true}"#;

        // act
        let result = parse(json).unwrap();

        // assert
        assert_eq!(result.content, "true");
    }

    #[test]
    fn test_deserialize_object_returns_error() {
        // arrange
        let json = r#"{"content": {"key": "value"}}"#;

        // act
        let result = parse(json);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_single_element_array() {
        // arrange
        let json = r#"{"content": ["only one"]}"#;

        // act
        let result = parse(json).unwrap();

        // assert
        assert_eq!(result.content, "only one");
    }
}
