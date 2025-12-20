//! Generate Keywords Use Case
//!
//! Orchestrates AI-powered keywords game generation.
//! Uses shared input/validation from `super::shared`.

use super::shared::{validate_generation_input, GenerateGameOutput};
use crate::domain::intello::KeywordSet;
use crate::error::AppResult;
use crate::services::{GenerateContentInput, IntelloService};
use std::sync::Arc;
use tracing::{info, instrument};

// Re-export shared type for backwards compatibility
pub use super::shared::GenerateGameInput as GenerateKeywordsInput;

/// Output from keywords generation
pub type GenerateKeywordsOutput = GenerateGameOutput<KeywordSet>;

/// Use case for generating keywords sets via AI
pub struct GenerateKeywordsUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateKeywordsUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: GenerateKeywordsInput) -> AppResult<GenerateKeywordsOutput> {
        // Shared validation
        let total_token_count = validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            model = ?input.model,
            "Generating AI keywords via use case"
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
        let keywords_set = self
            .intello_service
            .generate_ai_keywords(&input.user_id, service_input)
            .await?;

        info!(set_id = %keywords_set.id, "Keywords set generated");

        Ok(GenerateKeywordsOutput {
            game_set: keywords_set,
            total_token_count,
            documents_processed: input.documents.len(),
        })
    }
}

