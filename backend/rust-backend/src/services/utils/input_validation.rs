//! Input validation for Study service

use crate::services::error_domain::StudyError;
use crate::services::games::qcm::domain::QcmQuestion;
use crate::services::types_domain::GenerateContentInput;
use crate::services::QcmSet;
use crate::services::StudyService;

/// Maximum token count for AI generation
const MAX_TOKEN_COUNT: u32 = 800_000;

/// Allowed values for num_questions
const VALID_QUESTION_COUNTS: [u8; 6] =
    [5, 10, 15, 20, 25, 30];

/// Maximum subjects per generation request
const MAX_SUBJECTS: usize = 3;

/// Required number of wrong answers per QCM question
const REQUIRED_WRONG_ANSWERS: usize = 3;

impl StudyService {
    /// Reject empty string fields
    pub(crate) fn validate_user_id(
        &self,
        user_id: &str,
    ) -> Result<(), StudyError> {
        if user_id.is_empty() {
            return Err(StudyError::validation(
                "user_id",
                "User ID is required",
            ));
        }
        Ok(())
    }

    /// Reject empty set ID
    pub(crate) fn validate_set_id(
        &self,
        set_id: &str,
    ) -> Result<(), StudyError> {
        if set_id.is_empty() {
            return Err(StudyError::validation(
                "id",
                "Set ID is required",
            ));
        }
        Ok(())
    }

    /// Validate QCM set structure and content
    pub(crate) fn validate_qcm_set(
        &self,
        qcm_set: &QcmSet,
    ) -> Result<(), StudyError> {
        self.validate_user_id(&qcm_set.user_id)?;
        self.validate_set_id(&qcm_set.id)?;
        validate_required(
            "name", &qcm_set.name,
        )?;
        validate_required(
            "description", &qcm_set.description,
        )?;
        if qcm_set.questions.is_empty() {
            return Err(StudyError::validation(
                "questions",
                "At least one question is required",
            ));
        }
        for (i, q) in
            qcm_set.questions.iter().enumerate()
        {
            validate_question(i, q)?;
        }
        Ok(())
    }

    /// Validate AI generation input parameters
    pub(crate) fn validate_generation_input(
        &self,
        input: &GenerateContentInput,
    ) -> Result<(), StudyError> {
        validate_required("name", &input.name)?;
        validate_required(
            "description", &input.description,
        )?;
        validate_token_count(&input.documents)?;
        validate_num_questions(
            input.num_questions,
        )?;
        if input.subjects.len() > MAX_SUBJECTS {
            return Err(StudyError::validation(
                "subjects",
                "Maximum 3 subjects allowed",
            ));
        }
        Ok(())
    }
}

/// Reject empty trimmed string fields
fn validate_required(
    field: &str,
    value: &str,
) -> Result<(), StudyError> {
    if value.trim().is_empty() {
        return Err(StudyError::validation(
            field,
            format!("{field} is required"),
        ));
    }
    Ok(())
}

/// Validate a single QCM question at index
fn validate_question(
    index: usize,
    question: &QcmQuestion,
) -> Result<(), StudyError> {
    let pos = index + 1;
    if question.id.is_empty() {
        return Err(StudyError::validation(
            "questions",
            format!("Question {pos} is missing an ID"),
        ));
    }
    if question.question.trim().is_empty() {
        return Err(StudyError::validation(
            "questions",
            format!(
                "Question {pos} is missing \
                 the question text"
            ),
        ));
    }
    if question.wrong_answers.len()
        != REQUIRED_WRONG_ANSWERS
    {
        return Err(StudyError::validation(
            "questions",
            format!(
                "Question {pos} must have exactly \
                 {REQUIRED_WRONG_ANSWERS} wrong answers"
            ),
        ));
    }
    if question.right_answer.trim().is_empty() {
        return Err(StudyError::validation(
            "questions",
            format!(
                "Question {pos} is missing \
                 the right answer"
            ),
        ));
    }
    Ok(())
}

/// Validate total token count is within limit
fn validate_token_count(
    documents: &[(String, String, u32)],
) -> Result<(), StudyError> {
    let total: u32 = documents
        .iter()
        .map(|(_, _, tokens)| tokens)
        .sum();
    if total > MAX_TOKEN_COUNT {
        return Err(StudyError::validation(
            "documents",
            format!(
                "Total token count ({total}) \
                 exceeds maximum ({MAX_TOKEN_COUNT})"
            ),
        ));
    }
    Ok(())
}

/// Validate num_questions is in allowed set
fn validate_num_questions(
    count: u8,
) -> Result<(), StudyError> {
    if !VALID_QUESTION_COUNTS.contains(&count) {
        return Err(StudyError::validation(
            "num_questions",
            format!(
                "Must be one of: \
                 {VALID_QUESTION_COUNTS:?}"
            ),
        ));
    }
    Ok(())
}
