//! Generate Open Questions Use Case
//!
//! Orchestrates AI-powered open question generation with caching for later grading.
//! Uses shared input/validation from `super::shared`.

use super::shared::{validate_generation_input, GenerateGameOutput};
use crate::domain::intello::OpenQuestionSet;
use crate::error::AppResult;
use crate::services::{GenerateContentInput, IntelloService};
use std::sync::Arc;
use tracing::{info, instrument};

// Re-export shared type for backwards compatibility
pub use super::shared::GenerateGameInput as GenerateOpenQuestionsInput;

/// Output from open question generation
pub type GenerateOpenQuestionsOutput = GenerateGameOutput<OpenQuestionSet>;

/// Use case for generating open question sets via AI
pub struct GenerateOpenQuestionsUseCase {
    intello_service: Arc<IntelloService>,
}

impl GenerateOpenQuestionsUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, name = %input.name))]
    pub async fn execute(&self, input: GenerateOpenQuestionsInput) -> AppResult<GenerateOpenQuestionsOutput> {
        // Shared validation
        let total_token_count = validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            model = ?input.model,
            documents = input.documents.len(),
            "Generating AI open questions via use case"
        );

        // Build source content for caching (used during grading)
        // This is specific to open questions for AI grading context
        let source_content = input.documents.iter()
            .map(|(filename, content, _)| format!("=== {} ===\n{}", filename, content))
            .collect::<Vec<_>>()
            .join("\n\n");

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
        let open_question_set = self
            .intello_service
            .generate_ai_open_questions(&input.user_id, service_input, source_content)
            .await?;

        info!(set_id = %open_question_set.id, "Open questions generated");

        Ok(GenerateOpenQuestionsOutput {
            game_set: open_question_set,
            total_token_count,
            documents_processed: input.documents.len(),
        })
    }
}

