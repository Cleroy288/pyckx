//! Get User Sets Use Case
//!
//! Unified use case for retrieving all user content sets.
//! Note: Prepared for handler migration, not yet connected.

#![allow(dead_code)]

use crate::domain::intello::{
    FillBlankSet, FlashcardSet, KeywordSet, OpenQuestionSet, OrderPhraseSet, QcmSet, TrueOrFalseSet,
};
use crate::error::{AppError, AppResult};
use crate::services::IntelloService;
use std::sync::Arc;
use tracing::{info, instrument};

/// Enum for specifying which type of sets to retrieve
#[derive(Debug, Clone, Copy)]
pub enum SetType {
    Qcm,
    OpenQuestion,
    Flashcard,
    TrueOrFalse,
    Keywords,
    OrderPhrase,
    FillBlank,
}

/// Output containing all user sets of a specific type
#[derive(Debug, Clone)]
pub enum UserSetsOutput {
    Qcm(Vec<QcmSet>),
    OpenQuestion(Vec<OpenQuestionSet>),
    Flashcard(Vec<FlashcardSet>),
    TrueOrFalse(Vec<TrueOrFalseSet>),
    Keywords(Vec<KeywordSet>),
    OrderPhrase(Vec<OrderPhraseSet>),
    FillBlank(Vec<FillBlankSet>),
}

/// Use case for retrieving user sets
pub struct GetUserSetsUseCase {
    intello_service: Arc<IntelloService>,
}

impl GetUserSetsUseCase {
    pub fn new(intello_service: Arc<IntelloService>) -> Self {
        Self { intello_service }
    }

    #[instrument(skip(self), fields(user_id = %user_id, set_type = ?set_type))]
    pub async fn execute(&self, user_id: &str, set_type: SetType) -> AppResult<UserSetsOutput> {
        // Validate user_id
        if user_id.is_empty() {
            return Err(AppError::validation("user_id", "User ID is required"));
        }

        let result = match set_type {
            SetType::Qcm => {
                let sets = self.intello_service.get_user_qcm_sets(user_id).await?;
                info!(count = sets.len(), "Retrieved QCM sets");
                UserSetsOutput::Qcm(sets)
            }
            SetType::OpenQuestion => {
                let sets = self.intello_service.get_user_open_question_sets(user_id).await?;
                info!(count = sets.len(), "Retrieved open question sets");
                UserSetsOutput::OpenQuestion(sets)
            }
            SetType::Flashcard => {
                let sets = self.intello_service.get_user_flashcard_sets(user_id).await?;
                info!(count = sets.len(), "Retrieved flashcard sets");
                UserSetsOutput::Flashcard(sets)
            }
            SetType::TrueOrFalse => {
                let sets = self.intello_service.get_user_true_false_sets(user_id).await?;
                info!(count = sets.len(), "Retrieved true/false sets");
                UserSetsOutput::TrueOrFalse(sets)
            }
            SetType::Keywords => {
                let sets = self.intello_service.get_user_keyword_sets(user_id).await?;
                info!(count = sets.len(), "Retrieved keywords sets");
                UserSetsOutput::Keywords(sets)
            }
            SetType::OrderPhrase => {
                let sets = self.intello_service.get_user_order_phrase_sets(user_id).await?;
                info!(count = sets.len(), "Retrieved order phrase sets");
                UserSetsOutput::OrderPhrase(sets)
            }
            SetType::FillBlank => {
                let sets = self.intello_service.get_user_fill_blank_sets(user_id).await?;
                info!(count = sets.len(), "Retrieved fill blank sets");
                UserSetsOutput::FillBlank(sets)
            }
        };

        Ok(result)
    }
}
