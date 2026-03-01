//! Intello domain enums
//!
//! Contains enumeration types for the Intello quiz system.

use serde::{Deserialize, Serialize};

/// Difficulty level for a QCM set
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_easy_serializes_lowercase() {
        // arrange / act
        let json = serde_json::to_string(&Level::Easy).unwrap();

        // assert
        assert_eq!(json, r#""easy""#);
    }

    #[test]
    fn test_level_medium_serializes_lowercase() {
        // arrange / act
        let json = serde_json::to_string(&Level::Medium).unwrap();

        // assert
        assert_eq!(json, r#""medium""#);
    }

    #[test]
    fn test_level_hard_serializes_lowercase() {
        // arrange / act
        let json = serde_json::to_string(&Level::Hard).unwrap();

        // assert
        assert_eq!(json, r#""hard""#);
    }

    #[test]
    fn test_level_deserializes_from_lowercase() {
        // arrange
        let cases = vec![
            (r#""easy""#, Level::Easy),
            (r#""medium""#, Level::Medium),
            (r#""hard""#, Level::Hard),
        ];

        for (json, expected) in cases {
            // act
            let level: Level =
                serde_json::from_str(json).unwrap();

            // assert
            assert_eq!(
                level, expected,
                "Failed for input: {}",
                json
            );
        }
    }

    #[test]
    fn test_level_equality() {
        // arrange / act / assert
        assert_eq!(Level::Easy, Level::Easy);
        assert_ne!(Level::Easy, Level::Hard);
    }
}
