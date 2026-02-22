//! Quick QCM generation — ephemeral, no DB storage
//!
//! Dedicated path for quick QCM: lighter input (no
//! name/description/instructions/subjects), own validation,
//! and no persistence.

use tracing::{debug, info, instrument};

use crate::infra::openrouter::DEFAULT_MODEL;
use crate::services::intello::ai_usage::ai_usage_domain::{
    feature_type, AiUsageInput,
};
use crate::services::intello::custom_question_domain::{
    CustomQuestion, CustomQuestionDocument, DocumentType,
};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::games::qcm::domain::{
    QcmQuestion, QcmSet,
};
use crate::services::intello::types_domain::{
    DocumentList, IntelloService,
};
use crate::services::intello::{Level, SetId};

use super::parser::parse_qcm_response;

/// Input for quick (ephemeral) QCM generation
#[derive(Debug, Clone)]
pub struct QuickQcmInput {
    /// Target language code (e.g. "en", "fr")
    pub language: String,
    /// Difficulty level
    pub level: Level,
    /// Number of questions to generate
    pub num_questions: u8,
    /// (filename, content, token_count) tuples
    pub documents: DocumentList,
}

/// Max total tokens across all documents
const MAX_TOKEN_COUNT: u32 = 800_000;

/// Allowed question counts
const VALID_COUNTS: [u8; 6] = [5, 10, 15, 20, 25, 30];

/// Validate quick QCM input fields
fn validate_quick_input(
    input: &QuickQcmInput,
) -> Result<(), IntelloError> {
    if input.documents.is_empty() {
        return Err(IntelloError::validation(
            "documents",
            "At least one document is required",
        ));
    }
    let total: u32 =
        input.documents.iter().map(|d| d.2).sum();
    if total > MAX_TOKEN_COUNT {
        return Err(IntelloError::validation(
            "documents",
            format!(
                "Token count ({total}) exceeds max \
                 ({MAX_TOKEN_COUNT})"
            ),
        ));
    }
    if !VALID_COUNTS.contains(&input.num_questions) {
        return Err(IntelloError::validation(
            "num_questions",
            format!(
                "Must be one of: {:?}",
                VALID_COUNTS
            ),
        ));
    }
    Ok(())
}

/// Build CustomQuestion with quick-mode defaults
fn build_quick_question(
    user_id: &str,
    input: &QuickQcmInput,
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
        name: "Quick QCM".to_string(),
        description: "Quick session".to_string(),
        instructions: String::new(),
        language: input.language.clone(),
        level: input.level.clone(),
        output_game: "qcm".to_string(),
        subjects: Vec::new(),
        num_questions: input.num_questions,
        documents,
        total_token_count: input
            .documents
            .iter()
            .map(|d| d.2)
            .sum(),
    }
}

impl IntelloService {
    /// Generate ephemeral QCM (no DB persistence)
    #[instrument(
        skip(self, input),
        fields(user_id = %user_id)
    )]
    pub async fn generate_quick_qcm(
        &self,
        user_id: &str,
        input: QuickQcmInput,
    ) -> Result<QcmSet, IntelloError> {
        self.validate_user_id(user_id)?;
        validate_quick_input(&input)?;

        let custom_q =
            build_quick_question(user_id, &input);
        let prompt =
            super::prompt::build_qcm_prompt(&custom_q);

        let ai = self
            .openrouter_client
            .send_chat_request(&prompt, None)
            .await?;
        debug!(len = ai.content.len(), "AI response");

        let mut questions =
            parse_qcm_response(&ai.content)?;
        // Enforce requested count (AI may over-generate)
        questions.truncate(input.num_questions as usize);

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
            "Quick QCM generated"
        );
        Ok(build_quick_set(
            user_id, input, questions,
        ))
    }
}

/// Assemble the ephemeral QcmSet (no DB id needed)
fn build_quick_set(
    user_id: &str,
    input: QuickQcmInput,
    questions: Vec<QcmQuestion>,
) -> QcmSet {
    QcmSet {
        id: SetId::new(),
        user_id: user_id.into(),
        name: "Quick QCM".into(),
        description: String::new(),
        level: input.level,
        language: input.language,
        subjects: Vec::new(),
        questions,
    }
}
