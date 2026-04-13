//! Open-question API client — list, AI generation, grading.
//!
//! Thin wrapper around `crate::api::helpers` and the global
//! endpoint table. Returns `Result<_, String>` so callers
//! can surface errors via the toast layer.

use super::types::{
    CheckAnswersRequest, CheckAnswersResponse,
    CreateOpenQuestionResponse, OpenQuestionSet,
    OpenQuestionSetListResponse,
};
use crate::api::{endpoints, helpers};

/// Fetch every open-question set owned by the current user.
pub async fn list_sets(
) -> Result<Vec<OpenQuestionSet>, String> {
    helpers::get_sets::<OpenQuestionSetListResponse>(
        endpoints::OPEN_QUESTIONS,
    )
    .await
}

/// Create a new set via AI from an uploaded multipart form.
pub async fn create_set(
    form: &web_sys::FormData,
) -> Result<CreateOpenQuestionResponse, String> {
    helpers::post_multipart(
        endpoints::OPEN_QUESTIONS,
        form,
    )
    .await
}

/// Submit user answers for AI grading.
pub async fn grade_answers(
    req: &CheckAnswersRequest,
) -> Result<CheckAnswersResponse, String> {
    helpers::post_json(
        endpoints::OPEN_QUESTIONS_CHECK,
        req,
    )
    .await
}

/// Find a single set by ID inside the user's collection.
///
/// Pulls the full list and filters client-side; matches the
/// existing pattern used elsewhere in the app.
pub async fn find_set_by_id(
    id: String,
) -> Option<OpenQuestionSet> {
    list_sets()
        .await
        .ok()?
        .into_iter()
        .find(|s| s.id == id)
}
