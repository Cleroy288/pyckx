//! Keywords operations

use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::KeywordSet;
use crate::error::IntelloError;
use crate::shared::KeywordsPromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all keyword sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_keyword_sets(&self, user_id: &str) -> Result<Vec<KeywordSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        let sets = self.keywords_repo.find_by_user(user_id).await?;
        info!(count = sets.len(), "Retrieved user keyword sets");
        Ok(sets)
    }

    /// Get a specific keyword set by ID
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn get_keyword_set(&self, set_id: &str, user_id: &str) -> Result<Option<KeywordSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;
        self.keywords_repo.find_by_id(set_id, user_id).await
    }

    /// Delete a keyword set with ownership verification
    #[allow(dead_code)] // Available for future use
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_keyword_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        // Verify ownership
        let existing = self.keywords_repo.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            tracing::warn!(set_id = %set_id, "Keyword set not found or user doesn't own it");
            return Ok(false);
        }

        let deleted = self.keywords_repo.delete(set_id, user_id).await?;
        if deleted {
            info!(set_id = %set_id, "Keyword set deleted");
        }
        Ok(deleted)
    }

    /// Generate AI keyword questions and store them
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_keywords(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<KeywordSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(num_questions = input.num_questions, model = ?input.model, "Generating AI keyword questions");

        // Build prompt input
        let prompt_input = KeywordsPromptInput {
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
        let questions = self.openrouter_service.generate_keywords(&prompt_input, input.model.as_deref()).await?;

        info!(generated = questions.len(), "AI keyword questions generated");

        // Build and store the set
        let keyword_set = KeywordSet {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        };

        let created = self.keywords_repo.insert(&keyword_set).await?;
        info!(set_id = %created.id, "AI keyword set stored");

        Ok(created)
    }
}


