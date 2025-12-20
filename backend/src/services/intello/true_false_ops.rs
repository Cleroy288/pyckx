//! True or False operations

use super::crud_ops;
use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::TrueOrFalseSet;
use crate::error::IntelloError;
use crate::shared::TrueOrFalsePromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all true/false sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_true_false_sets(&self, user_id: &str) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        crud_ops::get_user_sets(self.true_false_repo.as_ref(), user_id, "true_false").await
    }

    /// Get a specific true/false set by ID
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn get_true_false_set(&self, set_id: &str, user_id: &str) -> Result<Option<TrueOrFalseSet>, IntelloError> {
        crud_ops::get_set(self.true_false_repo.as_ref(), set_id, user_id).await
    }

    /// Delete a true/false set with ownership verification
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_true_false_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        crud_ops::delete_set(self.true_false_repo.as_ref(), set_id, user_id, "true_false").await
    }

    /// Generate AI true/false statements and store them
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_true_false(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<TrueOrFalseSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(num_statements = input.num_questions, model = ?input.model, "Generating AI true/false statements");

        // Build prompt input
        let prompt_input = TrueOrFalsePromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_statements: input.num_questions,
            documents: input.documents.iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Generate statements via AI
        let statements = self.openrouter_service.generate_true_false(&prompt_input, input.model.as_deref()).await?;

        info!(generated = statements.len(), "AI true/false statements generated");

        // Build and store the set
        let true_false_set = TrueOrFalseSet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            statements,
        };

        let created = self.true_false_repo.insert(&true_false_set).await?;
        info!(set_id = %created.id, "AI true/false set stored");

        Ok(created)
    }
}
