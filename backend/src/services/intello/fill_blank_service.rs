//! Fill Blank operations

use super::crud_service;
use super::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::ai_usage_domain::feature_type;
use crate::services::intello::fill_blank_domain::FillBlankSet;
use crate::services::intello::SetId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::openrouter::DEFAULT_MODEL;
use super::prompt_builder_service::FillBlankPromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all fill blank sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_fill_blank_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<FillBlankSet>, IntelloError> {
        crud_service::get_user_sets(self.fill_blank_repo.as_ref(), user_id, "fill_blank").await
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

        info!(
            num_questions = input.num_questions,
            "Generating AI fill blank questions"
        );

        // Build prompt input
        let prompt_input = FillBlankPromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_questions: input.num_questions,
            documents: input
                .documents
                .iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Generate questions via AI
        let (questions, usage) = self
            .openrouter_service
            .generate_fill_blank(&prompt_input, None)
            .await?;

        // Log AI usage (fire-and-forget)
        if let Some(usage) = usage {
            self.try_log_ai_usage(
                user_id,
                DEFAULT_MODEL,
                feature_type::FILL_BLANK,
                usage.prompt_tokens,
                usage.completion_tokens,
            )
            .await;
        }

        info!(
            generated = questions.len(),
            "AI fill blank questions generated"
        );

        // Build and store the set
        let fill_blank_set = FillBlankSet {
            id: SetId::new(),
            user_id: user_id.into(),
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
