//! QCM set operations

use tracing::{debug, info, instrument, warn};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::intello::ai_usage::ai_usage_domain::feature_type;
use crate::services::intello::custom_question_domain::{
    CustomQuestion, CustomQuestionDocument, DocumentType,
};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::SetId;

use super::domain::QcmSet;
use super::parser::parse_qcm_response;

impl IntelloService {
    // ** create_qcm_set **
    // ==> Creates a new manual QCM set and stores it
    //
    // @ qcm_set : The QCM set to create
    // @ returns : The created QcmSet with assigned ID
    // @ errors : ValidationFailed if set data is invalid, StorageError if insert fails
    #[instrument(skip(self, qcm_set), fields(user_id = %qcm_set.user_id, set_id = %qcm_set.id))]
    pub async fn create_qcm_set(&self, qcm_set: QcmSet) -> Result<QcmSet, IntelloError> {
        // Step 1: Validate QCM set structure and content
        self.validate_qcm_set(&qcm_set)?;

        // Step 2: Insert into repository
        let created = self.qcm_repo.insert(&qcm_set).await?;

        // Step 3: Log success and return created set
        info!(set_id = %created.id, "QCM set created");
        Ok(created)
    }

    // ** get_user_qcm_sets **
    // ==> Retrieves all QCM sets for a user (manual + AI-generated)
    //
    // @ user_id : The user ID to query sets for
    // @ returns : Vector of all QcmSets owned by the user
    // @ errors : ValidationFailed if user_id invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id))]
    pub async fn get_user_qcm_sets(&self, user_id: &str) -> Result<Vec<QcmSet>, IntelloError> {
        // Step 1: Validate user ID
        self.validate_user_id(user_id)?;

        // Step 2: Query repository for all user sets
        // Note: Both qcm_repo and ai_qcm_repo use the same table (qcm_sets)
        let all_sets = self.qcm_repo.find_by_user(user_id).await?;

        // Step 3: Log retrieval count and return sets
        info!(count = all_sets.len(), "Retrieved user QCM sets");
        Ok(all_sets)
    }

    // ** get_qcm_set **
    // ==> Retrieves a specific QCM set by ID with ownership verification
    //
    // @ set_id : The ID of the set to retrieve
    // @ user_id : The user ID requesting the set
    // @ returns : Option<QcmSet> - Some if found and owned, None otherwise
    // @ errors : ValidationFailed if IDs invalid, StorageError if query fails
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn get_qcm_set(
        &self,
        set_id: &str,
        user_id: &str,
    ) -> Result<Option<QcmSet>, IntelloError> {
        // Step 1: Validate user and set IDs
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        // Step 2: Query repository with ownership check
        let set = self.qcm_repo.find_by_id(set_id, user_id).await?;

        // Step 3: Log if found and return result
        if set.is_some() {
            debug!(set_id = %set_id, "QCM set found");
        }
        Ok(set)
    }

    // ** update_qcm_set **
    // ==> Updates an existing QCM set with ownership verification
    //
    // @ qcm_set : The updated QCM set to save
    // @ returns : bool - true if updated, false if not found or not owned
    // @ errors : ValidationFailed if set invalid, StorageError if update fails
    #[instrument(skip(self, qcm_set), fields(user_id = %qcm_set.user_id, set_id = %qcm_set.id))]
    pub async fn update_qcm_set(&self, qcm_set: QcmSet) -> Result<bool, IntelloError> {
        // Step 1: Validate QCM set structure
        self.validate_qcm_set(&qcm_set)?;

        // Step 2: Verify user owns the set
        let existing = self
            .qcm_repo
            .find_by_id(&qcm_set.id, &qcm_set.user_id)
            .await?;
        if existing.is_none() {
            warn!(set_id = %qcm_set.id, "QCM set not found or user doesn't own it");
            return Ok(false);
        }

        // Step 3: Update the set in repository
        let updated = self.qcm_repo.update(&qcm_set).await?;

        // Step 4: Log success and return result
        if updated {
            info!(set_id = %qcm_set.id, "QCM set updated");
        }
        Ok(updated)
    }

    // ** delete_qcm_set **
    // ==> Deletes a QCM set with ownership verification
    //
    // @ set_id : The ID of the set to delete
    // @ user_id : The user requesting deletion
    // @ returns : bool - true if deleted, false if not found or not owned
    // @ errors : ValidationFailed if IDs invalid, StorageError if delete fails
    #[instrument(skip(self), fields(user_id = %user_id, set_id = %set_id))]
    pub async fn delete_qcm_set(&self, set_id: &str, user_id: &str) -> Result<bool, IntelloError> {
        // Step 1: Validate user and set IDs
        self.validate_user_id(user_id)?;
        self.validate_set_id(set_id)?;

        // Step 2: Verify user owns the set
        let existing = self.qcm_repo.find_by_id(set_id, user_id).await?;
        if existing.is_none() {
            warn!(set_id = %set_id, "QCM set not found or user doesn't own it");
            return Ok(false);
        }

        // Step 3: Delete from repository
        let deleted = self.qcm_repo.delete(set_id, user_id).await?;

        // Step 4: Log success and return result
        if deleted {
            info!(set_id = %set_id, "QCM set deleted");
        }
        Ok(deleted)
    }

    // ** generate_ai_qcm **
    // ==> Generates QCM questions using AI and stores them as a set
    //
    // @ user_id : The user requesting generation
    // @ input : Generation parameters (name, level, documents, etc.)
    // @ returns : The created QcmSet with AI-generated questions
    // @ errors : ValidationFailed if inputs invalid, ExternalServiceError if AI fails
    #[instrument(skip(self, input), fields(user_id = %user_id, name = %input.name))]
    pub async fn generate_ai_qcm(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<QcmSet, IntelloError> {
        // Step 1: Validate user ID and generation input
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;
        info!(
            num_questions = input.num_questions,
            "Generating AI QCM questions"
        );

        // Step 2: Build AI prompt from input
        let custom_question = self.build_custom_question_for_qcm(user_id, &input);
        let prompt = super::prompt::build_qcm_prompt(&custom_question);

        // Step 3: Send request to AI service
        let ai_result = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;
        debug!(
            response_length = ai_result.content.len(),
            "Received AI response"
        );

        // Step 4: Parse AI response into questions
        let questions = parse_qcm_response(&ai_result.content)?;

        // Step 5: Log AI usage (fire-and-forget)
        if let Some(usage) = ai_result.usage {
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

        // Step 6: Build and store the QCM set
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

    // ** build_custom_question_for_qcm **
    // ==> Builds a CustomQuestion structure from GenerateContentInput for QCM
    //
    // @ user_id : The user requesting generation
    // @ input : Generation input parameters
    // @ returns : CustomQuestion ready for prompt builder
    fn build_custom_question_for_qcm(
        &self,
        user_id: &str,
        input: &GenerateContentInput,
    ) -> crate::services::intello::CustomQuestion {
        // Build custom question with all input parameters
        CustomQuestion {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: input.name.clone(),
            description: input.description.clone(),
            instructions: input.instructions.clone(),
            language: input.language.clone(),
            level: input.level.clone(),
            output_game: "qcm".to_string(),
            subjects: input.subjects.clone(),
            num_questions: input.num_questions,
            documents: input
                .documents
                .iter()
                .map(|(filename, content, token_count)| CustomQuestionDocument {
                    filename: filename.clone(),
                    doc_type: DocumentType::Text,
                    content: content.clone(),
                    token_count: *token_count,
                })
                .collect(),
            total_token_count: input.documents.iter().map(|(_, _, t)| t).sum(),
        }
    }
}
