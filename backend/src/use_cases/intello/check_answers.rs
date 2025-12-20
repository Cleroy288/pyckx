//! Check Open Question Answers Use Case
//!
//! Orchestrates AI-powered grading of open question answers.

use crate::error::{AppError, AppResult};
use crate::services::{CheckAnswersInput, GradingResult, IntelloAnswerGrade, IntelloService, UserAnswer};
use std::sync::Arc;
use tracing::{info, instrument};

/// Input for checking answers
#[derive(Debug, Clone)]
pub struct CheckAnswersUseCaseInput {
    pub user_id: String,
    pub set_id: String,
    pub answers: Vec<UserAnswerInput>,
}

/// User answer input
#[derive(Debug, Clone)]
pub struct UserAnswerInput {
    pub question_id: String,
    pub user_answer: String,
}

/// Output from answer checking
#[derive(Debug, Clone)]
pub struct CheckAnswersOutput {
    pub results: Vec<GradingResultOutput>,
}

/// Individual grading result
#[derive(Debug, Clone)]
pub struct GradingResultOutput {
    pub question_id: String,
    pub grade: String,
    pub feedback: String,
}

impl From<GradingResult> for GradingResultOutput {
    fn from(result: GradingResult) -> Self {
        Self {
            question_id: result.question_id,
            grade: match result.grade {
                IntelloAnswerGrade::Right => "right".to_string(),
                IntelloAnswerGrade::Medium => "medium".to_string(),
                IntelloAnswerGrade::Error => "error".to_string(),
            },
            feedback: result.feedback,
        }
    }
}

/// Use case for checking/grading open question answers
pub struct CheckOpenQuestionAnswersUseCase {
    intello_service: Arc<IntelloService>,
}

impl CheckOpenQuestionAnswersUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self, input), fields(user_id = %input.user_id, set_id = %input.set_id))]
    pub async fn execute(&self, input: CheckAnswersUseCaseInput) -> AppResult<CheckAnswersOutput> {
        // 1. Validate answers
        if input.answers.is_empty() {
            return Err(AppError::validation("answers", "At least one answer is required"));
        }

        // 2. Validate set_id
        if input.set_id.is_empty() {
            return Err(AppError::validation("set_id", "Set ID is required"));
        }

        info!(
            count = input.answers.len(),
            "Checking open question answers via use case"
        );

        // 3. Convert input to service format
        let service_input = CheckAnswersInput {
            set_id: input.set_id,
            answers: input.answers.into_iter()
                .map(|a| UserAnswer {
                    question_id: a.question_id,
                    user_answer: a.user_answer,
                })
                .collect(),
        };

        // 4. Delegate to service
        let results = self
            .intello_service
            .check_open_question_answers(&input.user_id, service_input)
            .await?;

        info!(graded = results.len(), "Answers graded successfully");

        Ok(CheckAnswersOutput {
            results: results.into_iter().map(GradingResultOutput::from).collect(),
        })
    }
}
