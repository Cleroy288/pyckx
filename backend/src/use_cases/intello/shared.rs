//! Shared types and validation for Intello use cases
//!
//! This module provides common input/output types and validation logic
//! used across all AI-powered game generation use cases.

use crate::domain::intello::Level;
use crate::error::{AppError, AppResult};
use crate::services::{validate_model, validate_token_count, DEFAULT_MODEL};
use tracing::info;

// == SHARED INPUT TYPE ==

/// Common input for all AI game generation use cases.
///
/// This struct consolidates fields shared by QCM, Flashcard, OpenQuestion,
/// TrueOrFalse, Keywords, OrderPhrase, and FillBlank generation.
///
/// # Example
/// ```ignore
/// let input = GenerateGameInput {
///     user_id: "user-123".into(),
///     name: "My Quiz".into(),
///     description: "A quiz about Rust".into(),
///     // ...
/// };
/// let token_count = validate_generation_input(&input)?;
/// ```
#[derive(Debug, Clone)]
pub struct GenerateGameInput {
    /// User ID (required)
    pub user_id: String,
    /// Name of the game set
    pub name: String,
    /// Description of what this generates
    pub description: String,
    /// Specific instructions for the AI model
    pub instructions: String,
    /// Language for generated content (e.g., "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Subjects/topics for the content (max 3)
    pub subjects: Vec<String>,
    /// Number of questions/cards to generate (5, 10, 15, 20, 25, 30)
    pub num_questions: u8,
    /// Document contents: (filename, content, token_count)
    pub documents: Vec<(String, String, u32)>,
    /// AI model to use (None = default)
    pub model: Option<String>,
}

// == SHARED OUTPUT TYPE ==

/// Generic output from game set generation.
///
/// The type parameter `T` is the specific game set type (e.g., QcmSet, KeywordSet).
#[derive(Debug, Clone)]
pub struct GenerateGameOutput<T> {
    /// The generated game set
    pub game_set: T,
    /// Total tokens processed from input documents
    pub total_token_count: u32,
    /// Number of documents processed
    pub documents_processed: usize,
}

// == SHARED VALIDATION ==

/// Validate generation input fields (shared logic for all game types).
///
/// Performs the following validations:
/// 1. Model validation (if provided)
/// 2. Token count validation against model limits
/// 3. Subjects validation (at least one required)
/// 4. num_questions validation (1-50 range)
///
/// # Returns
/// - `Ok(total_token_count)` if validation passes
/// - `Err(AppError)` if any validation fails
///
/// # Example
/// ```ignore
/// let input = GenerateGameInput { ... };
/// let total_tokens = validate_generation_input(&input)?;
/// info!(tokens = total_tokens, "Validated input");
/// ```
pub fn validate_generation_input(input: &GenerateGameInput) -> AppResult<u32> {
    // 1. Validate model if provided
    if let Some(ref model) = input.model {
        validate_model(model).map_err(|e| AppError::validation("model", e))?;
    }

    // 2. Calculate and validate token count
    let total_token_count: u32 = input.documents.iter().map(|(_, _, t)| t).sum();
    let model_to_check = input.model.as_deref().unwrap_or(DEFAULT_MODEL);
    validate_token_count(model_to_check, total_token_count)
        .map_err(|e| AppError::validation("token_count", e))?;

    // 3. Validate subjects (at least one required)
    if input.subjects.is_empty() {
        return Err(AppError::validation(
            "subjects",
            "At least one subject is required",
        ));
    }

    // 4. Validate num_questions
    if input.num_questions == 0 || input.num_questions > 50 {
        return Err(AppError::validation(
            "num_questions",
            "Must be between 1 and 50",
        ));
    }

    info!(
        model = ?input.model,
        total_tokens = total_token_count,
        subjects = input.subjects.len(),
        num_questions = input.num_questions,
        "Validated generation input"
    );

    Ok(total_token_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> GenerateGameInput {
        GenerateGameInput {
            user_id: "test-user".into(),
            name: "Test Quiz".into(),
            description: "A test description".into(),
            instructions: "".into(),
            language: "en".into(),
            level: Level::Medium,
            subjects: vec!["Rust".into()],
            num_questions: 10,
            documents: vec![("test.txt".into(), "content".into(), 100)],
            model: None,
        }
    }

    #[test]
    fn test_validate_valid_input() {
        let input = valid_input();
        let result = validate_generation_input(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn test_validate_empty_subjects() {
        let mut input = valid_input();
        input.subjects = vec![];
        let result = validate_generation_input(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("subject"));
    }

    #[test]
    fn test_validate_num_questions_zero() {
        let mut input = valid_input();
        input.num_questions = 0;
        let result = validate_generation_input(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("num_questions"));
    }

    #[test]
    fn test_validate_num_questions_too_high() {
        let mut input = valid_input();
        input.num_questions = 51;
        let result = validate_generation_input(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_invalid_model() {
        let mut input = valid_input();
        input.model = Some("invalid/model".into());
        let result = validate_generation_input(&input);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("model"));
    }
}
