//! Fill Blank operations

use super::crud_ops;
use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::FillBlankSet;
use crate::error::IntelloError;
use crate::shared::FillBlankPromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all fill blank sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_fill_blank_sets(&self, user_id: &str) -> Result<Vec<FillBlankSet>, IntelloError> {
        crud_ops::get_user_sets(self.fill_blank_repo.as_ref(), user_id, "fill_blank").await
    }

    /// Get a specific fill blank set by ID
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn get_fill_blank_set(&self, set_id: &str, user_id: &str) -> Result<Option<FillBlankSet>, IntelloError> {
        crud_ops::get_set(self.fill_blank_repo.as_ref(), set_id, user_id).await
    }

    /// Delete a fill blank set with ownership verification
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_fill_blank_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        crud_ops::delete_set(self.fill_blank_repo.as_ref(), set_id, user_id, "fill_blank").await
    }

    /// Generate AI fill blank questions and store them
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_fill_blank(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<FillBlankSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(num_questions = input.num_questions, model = ?input.model, "Generating AI fill blank questions");

        // Build prompt input
        let prompt_input = FillBlankPromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_questions: input.num_questions,
            documents: input.documents.iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Generate questions via AI
        let questions = self.openrouter_service.generate_fill_blank(&prompt_input, input.model.as_deref()).await?;

        info!(generated = questions.len(), "AI fill blank questions generated");

        // Build and store the set
        let fill_blank_set = FillBlankSet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        };

        let created = self.fill_blank_repo.insert(&fill_blank_set).await?;
        info!(set_id = %created.id, "AI fill blank set stored");

        Ok(created)
    }
}
