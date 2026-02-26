//! True/False API — list and AI generation

use super::{endpoints, helpers};
use crate::domain::true_false_types::*;

helpers::impl_sets_response!(
    TrueOrFalseSetListResponse, TrueOrFalseSet
);

/// Get all true/false sets
pub async fn get_true_false_sets(
) -> Result<Vec<TrueOrFalseSet>, String> {
    helpers::get_sets::<TrueOrFalseSetListResponse>(
        endpoints::TRUE_FALSE,
    )
    .await
}

/// Create true/false via AI (multipart upload)
pub async fn create_true_false(
    form: &web_sys::FormData,
) -> Result<CreateTrueOrFalseResponse, String> {
    helpers::post_multipart(
        endpoints::TRUE_FALSE,
        form,
    )
    .await
}
