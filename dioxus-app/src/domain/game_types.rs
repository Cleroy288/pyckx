//! Study shared types — Level enum, common fields

use serde::{Deserialize, Serialize};

/// Difficulty level for game sets
#[derive(
    Debug, Clone, Copy, PartialEq,
    Serialize, Deserialize, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl Level {
    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
        }
    }

    /// Parse from string (defaults to Medium)
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "easy" => Self::Easy,
            "hard" => Self::Hard,
            _ => Self::Medium,
        }
    }
}

/// Number of questions for AI generation
#[derive(Debug, Clone, Copy, Serialize)]
pub struct NumQuestions(pub u8);

impl Default for NumQuestions {
    fn default() -> Self {
        Self(10)
    }
}

/// API error with optional field
#[derive(Debug, Clone, Deserialize)]
pub struct StudyApiError {
    pub error: String,
    pub field: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_as_str_all_variants() {
        let cases = vec![
            (Level::Easy, "easy"),
            (Level::Medium, "medium"),
            (Level::Hard, "hard"),
        ];
        for (level, expected) in cases {
            assert_eq!(
                level.as_str(),
                expected,
                "Level::{level:?} should be {expected:?}"
            );
        }
    }

    #[test]
    fn test_level_parse_valid_inputs() {
        let cases = vec![
            ("easy", Level::Easy),
            ("medium", Level::Medium),
            ("hard", Level::Hard),
        ];
        for (input, expected) in cases {
            assert_eq!(
                Level::parse(input),
                expected,
                "parse({input:?})"
            );
        }
    }

    #[test]
    fn test_level_parse_case_insensitive() {
        let cases = vec![
            ("EASY", Level::Easy),
            ("Hard", Level::Hard),
            ("MEDIUM", Level::Medium),
            ("EaSy", Level::Easy),
        ];
        for (input, expected) in cases {
            assert_eq!(
                Level::parse(input),
                expected,
                "parse({input:?}) case-insensitive"
            );
        }
    }

    #[test]
    fn test_level_parse_unknown_defaults_medium() {
        let cases = vec!["", "invalid", "xyz", " "];
        for input in cases {
            assert_eq!(
                Level::parse(input),
                Level::Medium,
                "parse({input:?}) should default"
            );
        }
    }

    #[test]
    fn test_level_default_is_medium() {
        assert_eq!(Level::default(), Level::Medium);
    }

    #[test]
    fn test_level_roundtrip() {
        let cases = [Level::Easy, Level::Medium, Level::Hard];
        for level in cases {
            assert_eq!(
                Level::parse(level.as_str()),
                level,
                "roundtrip for {level:?}"
            );
        }
    }

    #[test]
    fn test_num_questions_default_is_ten() {
        assert_eq!(NumQuestions::default().0, 10);
    }

    #[test]
    fn test_study_api_error_with_field() {
        let json = r#"{
            "error": "too short",
            "field": "name"
        }"#;
        let e: StudyApiError =
            serde_json::from_str(json).unwrap();
        assert_eq!(e.error, "too short");
        assert_eq!(e.field, Some("name".to_string()));
    }

    #[test]
    fn test_study_api_error_without_field() {
        let json = r#"{"error": "unauthorized"}"#;
        let e: StudyApiError =
            serde_json::from_str(json).unwrap();
        assert_eq!(e.error, "unauthorized");
        assert!(e.field.is_none());
    }
}
