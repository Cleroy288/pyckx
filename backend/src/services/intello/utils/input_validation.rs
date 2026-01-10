//! Input validation for Intello service
//!
//! Contains validation functions for user inputs, IDs, and data structures.
//! These functions ensure data integrity before business logic execution.

use crate::services::intello::domain::types_domain::{GenerateContentInput, IntelloService};
use crate::services::intello::error_domain::IntelloError;
use crate::services::intello::QcmSet;

impl IntelloService {
    // ** validate_user_id **
    // ==> Validates that a user ID is not empty
    //
    // @ user_id : The user ID to validate
    // @ returns : Ok(()) if valid
    // @ errors : ValidationFailed if user_id is empty
    pub(crate) fn validate_user_id(&self, user_id: &str) -> Result<(), IntelloError> {
        // Step 1: Check if user ID is empty
        if user_id.is_empty() {
            return Err(IntelloError::validation("user_id", "User ID is required"));
        }
        // Step 2: Return success
        Ok(())
    }

    // ** validate_set_id **
    // ==> Validates that a set ID is not empty
    //
    // @ set_id : The set ID to validate
    // @ returns : Ok(()) if valid
    // @ errors : ValidationFailed if set_id is empty
    pub(crate) fn validate_set_id(&self, set_id: &str) -> Result<(), IntelloError> {
        // Step 1: Check if set ID is empty
        if set_id.is_empty() {
            return Err(IntelloError::validation("id", "Set ID is required"));
        }
        // Step 2: Return success
        Ok(())
    }

    // ** validate_qcm_set **
    // ==> Validates QCM set structure and content integrity
    //
    // @ qcm_set : The QCM set to validate
    // @ returns : Ok(()) if all validations pass
    // @ errors : ValidationFailed if any field is invalid
    pub(crate) fn validate_qcm_set(&self, qcm_set: &QcmSet) -> Result<(), IntelloError> {
        // Step 1: Validate user ID and set ID
        self.validate_user_id(&qcm_set.user_id)?;
        self.validate_set_id(&qcm_set.id)?;

        // Step 2: Validate set name and description
        if qcm_set.name.trim().is_empty() {
            return Err(IntelloError::validation("name", "Name is required"));
        }
        if qcm_set.description.trim().is_empty() {
            return Err(IntelloError::validation(
                "description",
                "Description is required",
            ));
        }

        // Step 3: Validate each question in the set
        for (i, question) in qcm_set.questions.iter().enumerate() {
            if question.id.is_empty() {
                return Err(IntelloError::validation(
                    "questions",
                    format!("Question {} is missing an ID", i + 1),
                ));
            }
            if question.question.trim().is_empty() {
                return Err(IntelloError::validation(
                    "questions",
                    format!("Question {} is missing the question text", i + 1),
                ));
            }
            if question.wrong_answers.len() != 3 {
                return Err(IntelloError::validation(
                    "questions",
                    format!("Question {} must have exactly 3 wrong answers", i + 1),
                ));
            }
            if question.right_answer.trim().is_empty() {
                return Err(IntelloError::validation(
                    "questions",
                    format!("Question {} is missing the right answer", i + 1),
                ));
            }
        }

        // Step 4: Return success if all validations passed
        Ok(())
    }

    // ** validate_generation_input **
    // ==> Validates AI generation input parameters for completeness and limits
    //
    // @ input : The generation input to validate
    // @ returns : Ok(()) if all validations pass
    // @ errors : ValidationFailed if any parameter is invalid or exceeds limits
    pub(crate) fn validate_generation_input(
        &self,
        input: &GenerateContentInput,
    ) -> Result<(), IntelloError> {
        // Step 1: Validate name and description
        if input.name.trim().is_empty() {
            return Err(IntelloError::validation("name", "Name is required"));
        }
        if input.description.trim().is_empty() {
            return Err(IntelloError::validation(
                "description",
                "Description is required",
            ));
        }

        // Step 2: Validate documents are provided
        if input.documents.is_empty() {
            return Err(IntelloError::validation(
                "documents",
                "At least one document is required",
            ));
        }

        // Step 3: Validate total token count limit (800k)
        let total_tokens: u32 = input.documents.iter().map(|(_, _, tokens)| tokens).sum();
        if total_tokens > 800_000 {
            return Err(IntelloError::validation(
                "documents",
                format!(
                    "Total token count ({}) exceeds maximum allowed (800,000)",
                    total_tokens
                ),
            ));
        }

        // Step 4: Validate num_questions is in allowed set
        let valid_counts = [5, 10, 15, 20, 25, 30];
        if !valid_counts.contains(&input.num_questions) {
            return Err(IntelloError::validation(
                "num_questions",
                format!("num_questions must be one of: {:?}", valid_counts),
            ));
        }

        // Step 5: Validate subjects limit (max 3)
        if input.subjects.len() > 3 {
            return Err(IntelloError::validation(
                "subjects",
                "Maximum 3 subjects allowed",
            ));
        }

        // Step 6: Return success if all validations passed
        Ok(())
    }
}
