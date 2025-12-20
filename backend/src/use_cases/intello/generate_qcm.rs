//! Generate QCM Use Case
//!
//! Orchestrates AI-powered QCM (Multiple Choice Question) generation.
//! Uses shared input/validation from `super::shared`.

use super::shared::{validate_generation_input, GenerateGameOutput};
use crate::domain::intello::QcmSet;
use crate::error::AppResult;
use crate::services::{GenerateContentInput, IntelloService};
use std::sync::Arc;
use tracing::{info, instrument};

// Re-export shared type as GenerateQcmInput for backwards compatibility
pub use super::shared::GenerateGameInput as GenerateQcmInput;

/// Output from QCM generation (type alias for backwards compatibility)
pub type GenerateQcmOutput = GenerateGameOutput<QcmSet>;

/// Use case for generating QCM sets via AI
///
/// This use case:
/// 1. Validates input using shared validation
/// 2. Delegates to IntelloService for AI generation
/// 3. Returns the generated QcmSet
pub struct GenerateQcmUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateQcmUseCase {
    /// Create a new GenerateQcmUseCase
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    /// Execute the QCM generation workflow
    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: GenerateQcmInput) -> AppResult<GenerateQcmOutput> {
        // Shared validation (model, tokens, subjects, num_questions)
        let total_token_count = validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            model = ?input.model,
            documents = input.documents.len(),
            total_tokens = total_token_count,
            "Generating AI QCM via use case"
        );

        // Build service input
        let service_input = GenerateContentInput {
            name: input.name,
            description: input.description,
            instructions: input.instructions,
            language: input.language,
            level: input.level,
            subjects: input.subjects,
            num_questions: input.num_questions,
            documents: input.documents.clone(),
            model: input.model,
        };

        // Delegate to service for AI generation
        let qcm_set = self
            .intello_service
            .generate_ai_qcm(&input.user_id, service_input)
            .await?;

        info!(set_id = %qcm_set.id, questions = qcm_set.questions.len(), "QCM generated successfully");

        Ok(GenerateQcmOutput {
            game_set: qcm_set,
            total_token_count,
            documents_processed: input.documents.len(),
        })
    }
}

