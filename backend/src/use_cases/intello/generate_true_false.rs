//! Generate True/False Use Case
//!
//! Orchestrates AI-powered true/false question generation.
//! Uses shared input/validation from `super::shared`.

use super::shared::{validate_generation_input, GenerateGameOutput};
use crate::domain::intello::TrueOrFalseSet;
use crate::error::AppResult;
use crate::services::{GenerateContentInput, IntelloService};
use std::sync::Arc;
use tracing::{info, instrument};

// Re-export shared type for backwards compatibility
pub use super::shared::GenerateGameInput as GenerateTrueFalseInput;

/// Output from true/false generation
pub type GenerateTrueFalseOutput = GenerateGameOutput<TrueOrFalseSet>;

/// Use case for generating true/false sets via AI
pub struct GenerateTrueFalseUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateTrueFalseUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: GenerateTrueFalseInput) -> AppResult<GenerateTrueFalseOutput> {
        // Shared validation
        let total_token_count = validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            model = ?input.model,
            documents = input.documents.len(),
            "Generating AI true/false via use case"
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

        // Delegate to service
        let true_false_set = self
            .intello_service
            .generate_ai_true_false(&input.user_id, service_input)
            .await?;

        info!(set_id = %true_false_set.id, "True/false set generated");

        Ok(GenerateTrueFalseOutput {
            game_set: true_false_set,
            total_token_count,
            documents_processed: input.documents.len(),
        })
    }
}

