//! QCM AI generation operations

use tracing::{debug, info, instrument};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::ai_usage::ai_usage_domain::{
    feature_type, AiUsageInput,
};
use crate::services::custom_question_domain::{
    CustomQuestion, CustomQuestionDocument, DocumentType,
};
use crate::services::error_domain::StudyError;
use crate::services::games::qcm::domain::{
    QcmQuestion, QcmSet,
};
use crate::services::types_domain::GenerateContentInput;
use crate::services::StudyService;
use crate::services::SetId;

use super::parser::parse_qcm_response;

/// Build a QcmSet from user, questions, and input
fn build_qcm_set(
    user_id: &str,
    questions: Vec<QcmQuestion>,
    input: GenerateContentInput,
) -> QcmSet {
    QcmSet {
        id: SetId::new(),
        user_id: user_id.into(),
        name: input.name,
        description: input.description,
        level: input.level,
        language: input.language,
        subjects: input.subjects,
        questions,
    }
}

impl StudyService {
    /// Generates QCM questions using AI, stores them
    #[instrument(
        skip(self, input),
        fields(
            user_id = %user_id,
            name = %input.name,
        )
    )]
    pub async fn generate_ai_qcm(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<QcmSet, StudyError> {
        let (questions, input) = self
            .generate_qcm_core(user_id, input)
            .await?;

        let qcm_set =
            build_qcm_set(user_id, questions, input);

        let created =
            self.ai_qcm_repo.insert(&qcm_set).await?;
        info!(
            set_id = %created.id,
            "AI QCM set stored"
        );
        Ok(created)
    }

    /// Shared core: validate, prompt, call AI, parse
    async fn generate_qcm_core(
        &self,
        user_id: &str,
        input: GenerateContentInput,
    ) -> Result<
        (Vec<QcmQuestion>, GenerateContentInput),
        StudyError,
    > {
        self.validate_user_id(user_id)?;
        self.validate_generation_input(&input)?;

        let custom_q = self
            .build_custom_question_for_qcm(
                user_id, &input,
            );
        let prompt = super::prompt::build_qcm_prompt(
            &custom_q,
        );

        let ai = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;
        debug!(len = ai.content.len(), "AI response");

        let questions =
            parse_qcm_response(&ai.content)?;

        if questions.is_empty() {
            return Err(StudyError::validation(
                "ai_response",
                "AI returned no questions",
            ));
        }

        if let Some(u) = ai.usage {
            self.try_log_ai_usage(AiUsageInput {
                user_id,
                model_id: DEFAULT_MODEL,
                feature_type: feature_type::QCM,
                input_tokens: u.prompt_tokens,
                output_tokens: u.completion_tokens,
            })
            .await;
        }
        info!(
            count = questions.len(),
            "QCM questions generated"
        );
        Ok((questions, input))
    }

    /// Build CustomQuestion from GenerateContentInput
    fn build_custom_question_for_qcm(
        &self,
        user_id: &str,
        input: &GenerateContentInput,
    ) -> CustomQuestion {
        let documents = input
            .documents
            .iter()
            .map(|(name, content, tokens)| {
                CustomQuestionDocument {
                    filename: name.clone(),
                    doc_type: DocumentType::Text,
                    content: content.clone(),
                    token_count: *tokens,
                }
            })
            .collect();

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
            documents,
            total_token_count: input
                .documents
                .iter()
                .map(|(_, _, t)| t)
                .sum(),
        }
    }
}
