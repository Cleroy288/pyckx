//! Generate Content Use Case
//!
//! Shared use case for AI content generation with common validation.
//! Note: This is a utility use case not directly connected to handlers.

#![allow(dead_code)]

use crate::domain::intello::Level;
use crate::error::{AppError, AppResult};
use crate::services::{validate_model, validate_token_count, DEFAULT_MODEL};
use tracing::{info, instrument};

/// Common input for AI content generation
#[derive(Debug, Clone)]
pub struct GenerateContentInput {
    pub user_id: String,
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub language: String,
    pub level: Level,
    pub subjects: Vec<String>,
    pub num_questions: u8,
    pub documents: Vec<(String, String, u32)>,
    pub model: Option<String>,
}

/// Validation result with computed values
#[derive(Debug, Clone)]
pub struct ValidatedInput {
    pub total_token_count: u32,
    pub documents_processed: usize,
    pub model_used: String,
}

/// Shared use case for validating AI content generation input
///
/// This use case provides common validation logic that can be used
/// by all specific generation use cases (QCM, flashcards, etc.)
pub struct GenerateContentUseCase;

impl GenerateContentUseCase {
    /// Validate input for content generation
    ///
    /// Returns computed values if validation passes
    #[instrument(skip(input), fields(user_id = %input.user_id, name = %input.name))]
    pub fn validate(input: &GenerateContentInput) -> AppResult<ValidatedInput> {
        // 1. Validate user_id
        if input.user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }

        // 2. Validate name
        if input.name.trim().is_empty() {
            return Err(AppError::validation("name", "Name is required"));
        }

        // 3. Validate model if provided
        if let Some(ref model) = input.model {
            validate_model(model).map_err(|e| AppError::validation("model", e))?;
        }

        // 4. Calculate and validate token count
        let total_token_count: u32 = input.documents.iter().map(|(_, _, t)| t).sum();
        let model_to_check = input.model.as_deref().unwrap_or(DEFAULT_MODEL);
        validate_token_count(model_to_check, total_token_count)
            .map_err(|e| AppError::validation("token_count", e))?;

        // 5. Validate subjects
        if input.subjects.is_empty() {
            return Err(AppError::validation("subjects", "At least one subject is required"));
        }

        // 6. Validate num_questions
        if input.num_questions == 0 {
            return Err(AppError::validation("num_questions", "Number of questions must be greater than 0"));
        }

        info!(
            total_tokens = total_token_count,
            documents = input.documents.len(),
            model = ?input.model,
            "Content generation input validated"
        );

        Ok(ValidatedInput {
            total_token_count,
            documents_processed: input.documents.len(),
            model_used: model_to_check.to_string(),
        })
    }
}
