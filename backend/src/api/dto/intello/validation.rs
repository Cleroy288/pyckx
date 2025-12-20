//! Shared validation for Intello DTOs
//!
//! Note: Some validation is now handled by Use Cases.

#![allow(dead_code)]

use crate::domain::intello::Level;

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
        _ => Err(format!("Invalid level: {}. Must be easy, medium, or hard", level)),
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
