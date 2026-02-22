//! Open Questions API — list, AI generation, grading

use super::{endpoints, helpers};
use crate::domain::open_question_types::*;

helpers::impl_sets_response!(
    OpenQuestionSetListResponse, OpenQuestionSet
);

/// Get all open question sets
pub async fn get_open_question_sets(
) -> Result<Vec<OpenQuestionSet>, String> {
    helpers::get_sets::<OpenQuestionSetListResponse>(
        endpoints::OPEN_QUESTIONS,
    )
    .await
}

/// Create open questions via AI (multipart upload)
pub async fn create_open_questions(
    form: &web_sys::FormData,
) -> Result<CreateOpenQuestionResponse, String> {
    helpers::post_multipart(
        endpoints::OPEN_QUESTIONS,
        form,
    )
    .await
}

/// Submit answers for AI grading
pub async fn check_answers(
    req: &CheckAnswersRequest,
) -> Result<CheckAnswersResponse, String> {
    helpers::post_json(
        endpoints::OPEN_QUESTIONS_CHECK,
        req,
    )
    .await
}
