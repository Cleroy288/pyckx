//! Validation helpers

use super::types::{GenerateContentInput, IntelloService};
use crate::domain::intello::QcmSet;
use crate::error::IntelloError;

impl IntelloService {
    pub(super) fn validate_user_id(&self, user_id: &str) -> Result<(), IntelloError> {
        if user_id.is_empty() {
            return Err(IntelloError::validation("user_id", "User ID is required"));
        }
        Ok(())
    }

    pub(super) fn validate_set_id(&self, set_id: &str) -> Result<(), IntelloError> {
        if set_id.is_empty() {
            return Err(IntelloError::validation("id", "Set ID is required"));
        }
        Ok(())
    }

    pub(super) fn validate_qcm_set(&self, qcm_set: &QcmSet) -> Result<(), IntelloError> {
        self.validate_user_id(&qcm_set.user_id)?;
        self.validate_set_id(&qcm_set.id)?;

        if qcm_set.name.trim().is_empty() {
            return Err(IntelloError::validation("name", "Name is required"));
        }
        if qcm_set.description.trim().is_empty() {
            return Err(IntelloError::validation("description", "Description is required"));
        }

        // Validate questions
        for (i, question) in qcm_set.questions.iter().enumerate() {
            if question.id.is_empty() {
                return Err(IntelloError::validation("questions", format!("Question {} is missing an ID", i + 1)));
            }
            if question.question.trim().is_empty() {
                return Err(IntelloError::validation("questions", format!("Question {} is missing the question text", i + 1)));
            }
            if question.wrong_answers.len() != 3 {
                return Err(IntelloError::validation("questions", format!("Question {} must have exactly 3 wrong answers", i + 1)));
            }
            if question.right_answer.trim().is_empty() {
                return Err(IntelloError::validation("questions", format!("Question {} is missing the right answer", i + 1)));
            }
        }

        Ok(())
    }

    pub(super) fn validate_generation_input(&self, input: &GenerateContentInput) -> Result<(), IntelloError> {
        if input.name.trim().is_empty() {
            return Err(IntelloError::validation("name", "Name is required"));
        }
        if input.description.trim().is_empty() {
            return Err(IntelloError::validation("description", "Description is required"));
        }
        if input.documents.is_empty() {
            return Err(IntelloError::validation("documents", "At least one document is required"));
        }

        // Validate token limit (800k)
        let total_tokens: u32 = input.documents.iter().map(|(_, _, tokens)| tokens).sum();
        if total_tokens > 800_000 {
            return Err(IntelloError::validation(
                "documents",
                format!("Total token count ({}) exceeds maximum allowed (800,000)", total_tokens),
            ));
        }

        // Validate num_questions
        let valid_counts = [5, 10, 15, 20, 25, 30];
        if !valid_counts.contains(&input.num_questions) {
            return Err(IntelloError::validation(
                "num_questions",
                format!("num_questions must be one of: {:?}", valid_counts),
            ));
        }

        // Validate subjects (max 3)
        if input.subjects.len() > 3 {
            return Err(IntelloError::validation("subjects", "Maximum 3 subjects allowed"));
        }

        Ok(())
    }

    /// Build a CustomQuestion from GenerateContentInput (for QCM generation)
    pub(super) fn build_custom_question(&self, user_id: &str, input: &GenerateContentInput) -> crate::domain::intello::CustomQuestion {
        use crate::domain::intello::{CustomQuestion, CustomQuestionDocument, DocumentType};

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
            documents: input.documents.iter().map(|(filename, content, token_count)| {
                CustomQuestionDocument {
                    filename: filename.clone(),
                    doc_type: DocumentType::Text, // Simplified - actual type determined during extraction
                    content: content.clone(),
                    token_count: *token_count,
                }
            }).collect(),
            total_token_count: input.documents.iter().map(|(_, _, t)| t).sum(),
        }
    }
}
