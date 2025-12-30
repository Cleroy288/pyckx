//! QCM set operations

use super::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::ai_usage_domain::feature_type;
use crate::services::intello::qcm_set_domain::QcmSet;
use crate::services::intello::SetId;
use crate::services::intello::error_domain::IntelloError;
use crate::services::openrouter::DEFAULT_MODEL;
use tracing::{debug, info, instrument, warn};

impl IntelloService {
    /// Create a new manual QCM set
    #[instrument(skip(self, qcm_set), fields(user_id = %qcm_set.user_id, set_id = %qcm_set.id))]
    pub async fn create_qcm_set(&self, qcm_set: QcmSet) -> Result<QcmSet, IntelloError> {
        self.validate_qcm_set(&qcm_set)?;
        let created = self.qcm_repo.insert(&qcm_set).await?;
        info!(set_id = %created.id, "QCM set created");
        Ok(created)
    }

    /// Get all QCM sets for a user (manual + AI-generated)
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_qcm_sets(&self, user_id: &str) -> Result<Vec<QcmSet>, IntelloError> {
        self.validate_user_id(user_id)?;

        // Note: Both qcm_repo and ai_qcm_repo use the same table (qcm_sets),
        // so we only need to query once to get all sets (manual + AI-generated).
        let all_sets = self.qcm_repo.find_by_user(user_id).await?;

        info!(count = all_sets.len(), "Retrieved user QCM sets");
        Ok(all_sets)
    }

    /// Get a specific QCM set by ID
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn get_qcm_set(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<QcmSet>, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        let set = self.qcm_repo.find_by_id(set_id, user_id).await?;
        if set.is_some() {
            debug!(set_id = %set_id, "QCM set found");
        }
        Ok(set)
    }

    /// Update a QCM set with ownership verification
    #[instrument(skip(self, qcm_set), fields(user_id = %qcm_set.user_id, set_id = %qcm_set.id))]
    pub async fn update_qcm_set(&self, qcm_set: QcmSet) -> Result<bool, IntelloError> {
        self.validate_qcm_set(&qcm_set)?;

        // Verify ownership
        let existing = self
            .qcm_repo
            .find_by_id(&qcm_set.id, &qcm_set.user_id)
            .await?;
        if existing.is_none() {
            warn!(set_id = %qcm_set.id, "QCM set not found or user doesn't own it");
            return Ok(false);
        }

        let updated = self.qcm_repo.update(&qcm_set).await?;
        if updated {
            info!(set_id = %qcm_set.id, "QCM set updated");
        }
        Ok(updated)
    }

    /// Delete a QCM set with ownership verification
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_qcm_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        // Verify ownership
        let existing = self.qcm_repo.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            warn!(set_id = %set_id, "QCM set not found or user doesn't own it");
            return Ok(false);
        }

        let deleted = self.qcm_repo.delete(set_id, user_id).await?;
        if deleted {
            info!(set_id = %set_id, "QCM set deleted");
        }
        Ok(deleted)
    }

    /// Generate AI QCM questions and store them
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_qcm(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<QcmSet, IntelloError> {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        info!(
            num_questions = input.num_questions,
            "Generating AI QCM questions"
        );

        // Build CustomQuestion for OpenRouter
        let custom_question = self.build_custom_question(user_id, &input);

        // Generate questions via AI
        let (questions, usage) = self
            .openrouter_service
            .generate_qcm(&custom_question, None)
            .await?;

        // Log AI usage (fire-and-forget)
        if let Some(usage) = usage {
            self.try_log_ai_usage(
                user_id,
                DEFAULT_MODEL,
                feature_type::QCM,
                usage.prompt_tokens,
                usage.completion_tokens,
            )
            .await;
        }

        info!(generated = questions.len(), "AI QCM questions generated");

        // Build and store the QCM set
        let qcm_set = QcmSet {
            id: SetId::new(),
            user_id: user_id.into(),
            name: input.name,
            description: input.description,
            level: input.level,
            language: input.language,
            subjects: input.subjects,
            questions,
        };

        let created = self.ai_qcm_repo.insert(&qcm_set).await?;
        info!(set_id = %created.id, "AI QCM set stored");

        Ok(created)
    }
}
