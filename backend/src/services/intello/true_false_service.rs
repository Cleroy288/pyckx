//! True or False operations

use super::crud_service;
use super::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::ai_usage_domain::feature_type;
use crate::services::intello::true_false_domain::TrueOrFalseSet;
use crate::services::intello::SetId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::openrouter::DEFAULT_MODEL;
use super::prompt_builder_service::TrueOrFalsePromptInput;
use tracing::{info, instrument};

impl IntelloService {
    /// Get all true/false sets for a user
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_true_false_sets(
        &self,
        user_id: &str,
    ) -> Result<Vec<TrueOrFalseSet>, IntelloError> {
        crud_service::get_user_sets(self.true_false_repo.as_ref(), user_id, "true_false").await
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

        info!(
            num_statements = input.num_questions,
            "Generating AI true/false statements"
        );

        // Build prompt input
        let prompt_input = TrueOrFalsePromptInput {
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            subjects: input.subjects.clone(),
            num_statements: input.num_questions,
            documents: input
                .documents
                .iter()
                .map(|(filename, content, _)| (filename.clone(), content.clone()))
                .collect(),
        };

        // Generate statements via AI
        let (statements, usage) = self
            .openrouter_service
            .generate_true_false(&prompt_input, None)
            .await?;

        // Log AI usage (fire-and-forget)
        if let Some(usage) = usage {
            self.try_log_ai_usage(
                user_id,
                DEFAULT_MODEL,
                feature_type::TRUE_FALSE,
                usage.prompt_tokens,
                usage.completion_tokens,
            )
            .await;
        }

        info!(
            generated = statements.len(),
            "AI true/false statements generated"
        );

        // Build and store the set
        let true_false_set = TrueOrFalseSet {
            id: SetId::new(),
            user_id: user_id.into(),
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
