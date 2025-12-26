//! Keywords operations

use super::crud_ops;
use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::KeywordSet;
use crate::error::IntelloError;
use crate::shared::KeywordsPromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all keyword sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_keyword_sets(&self, user_id: &str) -> Result<Vec<KeywordSet>, IntelloError> {
        crud_ops::get_user_sets(self.keywords_repo.as_ref(), user_id, "keywords").await
    }

    /// Delete a keyword set with ownership verification
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_keyword_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        crud_ops::delete_set(self.keywords_repo.as_ref(), set_id, user_id, "keywords").await
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


