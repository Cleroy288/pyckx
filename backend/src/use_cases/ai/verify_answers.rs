//! Verify Answers Use Case
//!
//! Shared use case for AI-powered answer verification.
//! Note: This is a utility use case not directly connected to handlers.

#![allow(dead_code)]

use crate::error::{AppError, AppResult};
use crate::services::OpenRouterService;
use crate::shared::{AnswerToGrade, VerificationPromptInput};
use std::sync::Arc;
use tracing::{info, instrument};

/// Input for answer verification
#[derive(Debug, Clone)]
pub struct VerifyAnswersInput {
    pub language: String,
    pub source_content: String,
    pub answers: Vec<AnswerToVerify>,
}

/// A single answer to verify
#[derive(Debug, Clone)]
pub struct AnswerToVerify {
    pub question_id: String,
    pub question: String,
    pub hint: String,
    pub expected_answer: String,
    pub user_answer: String,
}

/// Grade result from verification
#[derive(Debug, Clone)]
pub struct VerifiedAnswer {
    pub question_id: String,
    pub grade: String,  // "right", "medium", "error"
    pub feedback: String,
}

/// Use case for verifying user answers via AI
pub struct VerifyAnswersUseCase {
    openrouter_service: Arc<OpenRouterService>,
}

impl VerifyAnswersUseCase {
    pub fn new(openrouter_service: Arc<OpenRouterService>) -> Self {
        Self { openrouter_service }
    }

    #[instrument(skip(self, input), fields(answers = input.answers.len()))]
    pub async fn execute(&self, input: VerifyAnswersInput) -> AppResult<Vec<VerifiedAnswer>> {
        // 1. Validate input
        if input.answers.is_empty() {
            return Err(AppError::validation("answers", "At least one answer is required"));
        }

        if input.language.is_empty() {
            return Err(AppError::validation("language", "Language is required"));
        }

        info!(
            answers = input.answers.len(),
            "Verifying answers via AI"
        );

        // 2. Convert to service input format
        let answers_to_grade: Vec<AnswerToGrade> = input.answers.iter()
            .map(|a| AnswerToGrade {
                question_id: a.question_id.clone(),
                question: a.question.clone(),
                hint: a.hint.clone(),
                expected_answer: a.expected_answer.clone(),
                user_answer: a.user_answer.clone(),
            })
            .collect();

        let verification_input = VerificationPromptInput {
            language: input.language,
            source_content: input.source_content,
            answers: answers_to_grade,
        };

        // 3. Delegate to OpenRouter service
        let grades = self.openrouter_service.verify_answers(&verification_input).await?;

        // 4. Convert to output format
        let results: Vec<VerifiedAnswer> = grades.into_iter()
            .map(|g| VerifiedAnswer {
                question_id: g.question_id,
                grade: match g.grade {
                    crate::services::AnswerGrade::Right => "right".to_string(),
                    crate::services::AnswerGrade::Medium => "medium".to_string(),
                    crate::services::AnswerGrade::Error => "error".to_string(),
                },
                feedback: g.feedback,
            })
            .collect();

        info!(verified = results.len(), "Answers verified successfully");

        Ok(results)
    }
}
