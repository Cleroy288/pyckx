//! Generate Fill Blank Use Case
//!
//! Orchestrates AI-powered fill-in-the-blank game generation.
//! Uses shared input/validation from `super::shared`.

use super::shared::{validate_generation_input, GenerateGameOutput};
use crate::domain::intello::FillBlankSet;
use crate::error::AppResult;
use crate::services::{GenerateContentInput, IntelloService};
use std::sync::Arc;
use tracing::{info, instrument};

// Re-export shared type for backwards compatibility
pub use super::shared::GenerateGameInput as GenerateFillBlankInput;

/// Output from fill blank generation
pub type GenerateFillBlankOutput = GenerateGameOutput<FillBlankSet>;

/// Use case for generating fill blank sets via AI
pub struct GenerateFillBlankUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateFillBlankUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: GenerateFillBlankInput) -> AppResult<GenerateFillBlankOutput> {
        // Shared validation
        let total_token_count = validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            model = ?input.model,
            "Generating AI fill blank via use case"
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
        let fill_blank_set = self
            .intello_service
            .generate_ai_fill_blank(&input.user_id, service_input)
            .await?;

        info!(set_id = %fill_blank_set.id, "Fill blank set generated");

        Ok(GenerateFillBlankOutput {
            game_set: fill_blank_set,
            total_token_count,
            documents_processed: input.documents.len(),
        })
    }
}

