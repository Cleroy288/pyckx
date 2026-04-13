//! Shared validation for Study DTOs
//!
//! Note: Some validation is now handled by Use Cases.

#![allow(dead_code)]

use crate::services::Level;

/// Maximum number of subjects allowed per set
pub const MAX_SUBJECTS: usize = 3;
/// Maximum character length for each subject
pub const MAX_SUBJECT_LENGTH: usize = 20;
/// Valid number of questions options
pub const VALID_NUM_QUESTIONS: [u8; 6] = [5, 10, 15, 20, 25, 30];

/// Validate subjects: max 3 subjects, each max 20 characters
pub fn validate_subjects(subjects: &[String]) -> Result<(), String> {
    if subjects.len() > MAX_SUBJECTS {
        return Err(format!(
            "Too many subjects: {}. Maximum allowed is {}",
            subjects.len(),
            MAX_SUBJECTS
        ));
    }
    for (i, subject) in subjects.iter().enumerate() {
        if subject.len() > MAX_SUBJECT_LENGTH {
            return Err(format!(
                "Subject {} is too long ({} chars). Maximum is {} characters",
                i + 1,
                subject.len(),
                MAX_SUBJECT_LENGTH
            ));
        }
    }
    Ok(())
}

/// Parse level string to Level enum
pub fn parse_level(level: &str) -> Result<Level, String> {
    match level.to_lowercase().as_str() {
        "easy" => Ok(Level::Easy),
        "medium" => Ok(Level::Medium),
        "hard" => Ok(Level::Hard),
        _ => Err(format!(
            "Invalid level: {}. Must be easy, medium, or hard",
            level
        )),
    }
}

/// Validate number of questions
pub fn validate_num_questions(num: u8) -> Result<(), String> {
    if VALID_NUM_QUESTIONS.contains(&num) {
        Ok(())
    } else {
        Err(format!(
            "Invalid number of questions: {}. Must be one of: {:?}",
            num, VALID_NUM_QUESTIONS
        ))
    }
}

/// Default language
pub fn default_language() -> String {
    "en".to_string()
}

/// Default number of questions
pub fn default_num_questions() -> u8 {
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- validate_subjects --

    #[test]
    fn test_validate_subjects_empty_ok() {
        // arrange
        let subjects: Vec<String> = vec![];

        // act
        let result = validate_subjects(&subjects);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_subjects_within_limit_ok() {
        // arrange
        let subjects = vec!["Math".into(), "Science".into()];

        // act
        let result = validate_subjects(&subjects);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_subjects_too_many_returns_error() {
        // arrange
        let subjects: Vec<String> = vec![
            "A".into(),
            "B".into(),
            "C".into(),
            "D".into(),
        ];

        // act
        let result = validate_subjects(&subjects);

        // assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Too many"));
    }

    #[test]
    fn test_validate_subjects_too_long_returns_error() {
        // arrange
        let long = "a".repeat(MAX_SUBJECT_LENGTH + 1);
        let subjects = vec![long];

        // act
        let result = validate_subjects(&subjects);

        // assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too long"));
    }

    // -- parse_level --

    #[test]
    fn test_parse_level_easy() {
        // arrange / act
        let result = parse_level("easy");

        // assert
        assert_eq!(result.unwrap(), Level::Easy);
    }

    #[test]
    fn test_parse_level_medium_case_insensitive() {
        // arrange / act
        let result = parse_level("Medium");

        // assert
        assert_eq!(result.unwrap(), Level::Medium);
    }

    #[test]
    fn test_parse_level_hard_uppercase() {
        // arrange / act
        let result = parse_level("HARD");

        // assert
        assert_eq!(result.unwrap(), Level::Hard);
    }

    #[test]
    fn test_parse_level_invalid_returns_error() {
        // arrange / act
        let result = parse_level("expert");

        // assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expert"));
    }

    // -- validate_num_questions --

    #[test]
    fn test_validate_num_questions_valid_values() {
        // arrange
        let valid = vec![5, 10, 15, 20, 25, 30];

        // act / assert
        for n in valid {
            assert!(
                validate_num_questions(n).is_ok(),
                "Expected {} to be valid",
                n,
            );
        }
    }

    #[test]
    fn test_validate_num_questions_invalid_value() {
        // arrange / act
        let result = validate_num_questions(7);

        // assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("7"));
    }

    // -- defaults --

    #[test]
    fn test_default_language_is_en() {
        // arrange / act / assert
        assert_eq!(default_language(), "en");
    }

    #[test]
    fn test_default_num_questions_is_10() {
        // arrange / act / assert
        assert_eq!(default_num_questions(), 10);
    }
}
